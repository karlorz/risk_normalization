// risk_normalization.cpp

#include <iostream>
#include <vector>
#include <fstream>
#include <sstream>
#include <cmath>
#include <algorithm>
#include <numeric>
#include <random>
#include <iomanip>

// Function to read trades data from a CSV file
std::vector<double> readTradesFromCSV(const std::string& filename) {
    std::vector<double> trades;
    std::ifstream file(filename);
    if (!file.is_open()) {
        std::cerr << "Unable to open file: " << filename << std::endl;
        return trades;
    }
    std::string line;
    while (std::getline(file, line)) {
        std::stringstream ss(line);
        double value;
        try {
            value = std::stod(line);
            trades.push_back(value);
        } catch (...) {
            // Skip non-numeric lines
            continue;
        }
    }
    file.close();
    return trades;
}

// Function to compute mean of a vector
double computeMean(const std::vector<double>& data) {
    if (data.empty()) return 0.0;
    double sum = std::accumulate(data.begin(), data.end(), 0.0);
    return sum / data.size();
}

// Function to compute standard deviation of a vector
double computeStdDev(const std::vector<double>& data, double mean) {
    if (data.size() < 2) return 0.0;
    double accum = 0.0;
    for (double x : data) {
        accum += (x - mean) * (x - mean);
    }
    return std::sqrt(accum / (data.size() - 1));
}

// Function to calculate maximum drawdown from equity curve
double calculateDrawdown(const std::vector<double>& equity_curve) {
    double peak = equity_curve[0];
    double max_drawdown = 0.0;
    for (double equity : equity_curve) {
        if (equity > peak) {
            peak = equity;
        }
        double drawdown = (peak - equity) / peak;
        if (drawdown > max_drawdown) {
            max_drawdown = drawdown;
        }
    }
    return max_drawdown;
}

// Function to calculate CAGR
double calculateCAGR(double initial_equity, double final_equity, double years) {
    if (initial_equity <= 0 || final_equity <= 0 || years <= 0) return 0.0;
    return (std::pow(final_equity / initial_equity, 1.0 / years) - 1.0) * 100.0; // Return percentage
}

// Function to simulate one equity sequence and calculate max drawdown
std::pair<std::vector<double>, double> make_one_equity_sequence(
    const std::vector<double>& trades,
    double fraction,
    int number_trades_in_forecast,
    double initial_capital,
    std::mt19937& rng
) {
    std::vector<double> equity_curve(number_trades_in_forecast + 1);
    equity_curve[0] = initial_capital;
    std::uniform_int_distribution<> dist(0, trades.size() - 1);

    for (int i = 1; i <= number_trades_in_forecast; ++i) {
        int idx = dist(rng);
        double trade_return = trades[idx] * fraction * equity_curve[i - 1];
        equity_curve[i] = equity_curve[i - 1] + trade_return;
    }

    double max_drawdown = calculateDrawdown(equity_curve);
    return { equity_curve, max_drawdown };
}

// Function to analyze distribution of drawdowns and compute tail risk
double analyze_distribution_of_drawdown(
    const std::vector<double>& trades,
    double fraction,
    int number_trades_in_forecast,
    double initial_capital,
    double drawdown_tolerance,
    int number_equity_in_CDF,
    std::mt19937& rng
) {
    int count_exceed = 0;
    for (int i = 0; i < number_equity_in_CDF; ++i) {
        auto [equity, max_drawdown] = make_one_equity_sequence(
            trades,
            fraction,
            number_trades_in_forecast,
            initial_capital,
            rng
        );
        if (max_drawdown > drawdown_tolerance) {
            count_exceed++;
        }
    }
    return static_cast<double>(count_exceed) / number_equity_in_CDF;
}

// Function to compute mean and standard deviation
void computeStatistics(const std::vector<double>& data, double& mean, double& stdev) {
    mean = computeMean(data);
    stdev = computeStdDev(data, mean);
}

// risk_normalization function implementation
void risk_normalization(const std::vector<double>& trades,
                        int number_days_in_forecast,
                        int number_trades_in_forecast,
                        double initial_capital,
                        double tail_percentile,
                        double drawdown_tolerance,
                        int number_equity_in_CDF,
                        int number_repetitions,
                        double& safe_f_mean,
                        double& safe_f_stdev,
                        double& CAR25_mean,
                        double& CAR25_stdev) {
    std::vector<double> safe_f_list;
    std::vector<double> CAR25_list;
    double desired_accuracy = 0.003;

    std::mt19937 rng(std::random_device{}());

    for (int rep = 0; rep < number_repetitions; ++rep) {
        double fraction = 1.0;
        double tolerance = desired_accuracy; // Updated to match Python's desired_accuracy
        int max_iterations = 1000;
        int iteration = 0;
        bool done = false;

        // Bisection method to find safe_f
        double lower_bound = 0.0;
        double upper_bound = 10.0; // Arbitrary upper limit for fraction
        double tail_risk = 0.0;

        while (!done && iteration < max_iterations) {
            fraction = (lower_bound + upper_bound) / 2.0;
            tail_risk = analyze_distribution_of_drawdown(
                trades,
                fraction,
                number_trades_in_forecast,
                initial_capital,
                drawdown_tolerance,
                number_equity_in_CDF,
                rng
            );

            if (std::abs(tail_risk - (tail_percentile / 100.0)) < tolerance) {
                done = true;
            } else if (tail_risk > (tail_percentile / 100.0)) {
                upper_bound = fraction;
            } else {
                lower_bound = fraction;
            }
            iteration++;
        }

        safe_f_list.push_back(fraction);

        // Simulate equity curves to collect CARs
        std::vector<double> CAR_list;
        for (int i = 0; i < number_equity_in_CDF; ++i) {
            auto [equity_curve, max_drawdown] = make_one_equity_sequence(
                trades,
                fraction,
                number_trades_in_forecast,
                initial_capital,
                rng
            );

            double years = static_cast<double>(number_days_in_forecast) / 252.0;
            double CAGR = calculateCAGR(initial_capital, equity_curve.back(), years);
            CAR_list.push_back(CAGR);
        }

        // Calculate the 25th percentile CAR (CAR25)
        std::sort(CAR_list.begin(), CAR_list.end());
        size_t index = static_cast<size_t>(std::ceil(0.25 * CAR_list.size())) - 1;
        index = std::min(index, CAR_list.size() - 1);
        double CAR25 = CAR_list[index];
        CAR25_list.push_back(CAR25);

        // Print Compound Annual Return for this repetition
        std::cout << "Compound Annual Return: " << std::fixed << std::setprecision(5) << CAR25 << "%" << std::endl;
    }

    // Compute statistics for safe_f
    computeStatistics(safe_f_list, safe_f_mean, safe_f_stdev);

    // Compute statistics for CAR25
    computeStatistics(CAR25_list, CAR25_mean, CAR25_stdev);
}

int main() {
    // Read in the CSV file
    std::string file_name = "./data/generated_normal_trades.csv";
    std::cout << "The data file being processed is: " << file_name << std::endl;
    std::vector<double> trades = readTradesFromCSV(file_name);
    if (trades.empty()) {
        std::cerr << "No trades data found." << std::endl;
        return 1;
    }
    std::cout << "There are " << trades.size() << " marked-to-market daily trades in the file" << std::endl;
    std::cout << "Here are the first 10 trades:" << std::endl;
    for (size_t i = 0; i < std::min(trades.size(), static_cast<size_t>(10)); ++i) {
        std::cout << trades[i] << std::endl;
    }

    double number_of_years_in_CSV = 28.0;
    double average_trades_per_year = trades.size() / number_of_years_in_CSV;
    double years_to_forecast = 2.0;

    // Calculate number of days and trades in forecast period
    int number_days_in_forecast = static_cast<int>(years_to_forecast * 252);  // Assuming 252 trading days per year
    int number_trades_in_forecast = static_cast<int>(average_trades_per_year * years_to_forecast);

    double initial_capital = 100000.0;
    double tail_percentile = 5.0;
    double drawdown_tolerance = 0.10;
    int number_equity_in_CDF = 10000;
    int number_repetitions = 5;

    double safe_f_mean = 0.0;
    double safe_f_stdev = 0.0;
    double CAR25_mean = 0.0;
    double CAR25_stdev = 0.0;

    // Call risk_normalization function
    risk_normalization(trades,
                       number_days_in_forecast,
                       number_trades_in_forecast,
                       initial_capital,
                       tail_percentile,
                       drawdown_tolerance,
                       number_equity_in_CDF,
                       number_repetitions,
                       safe_f_mean,
                       safe_f_stdev,
                       CAR25_mean,
                       CAR25_stdev);

    std::cout << "CAR25 mean:   " << std::fixed << std::setprecision(5) << CAR25_mean << "%" << std::endl;
    std::cout << "CAR25 stdev:  " << std::fixed << std::setprecision(5) << CAR25_stdev << std::endl;
    std::cout << "safe-f mean:  " << std::fixed << std::setprecision(5) << safe_f_mean << std::endl;
    std::cout << "safe-f stdev: " << std::fixed << std::setprecision(5) << safe_f_stdev << std::endl;

    return 0;
}