# Risk normalization

The risk_normalization.py program analyzes the 'best estimate' set of trades associated with a trading system and computes the maximum position size that will limit the drawdown to within the trader's personal risk tolerance.

Traders are willing to trust the signals of their algorithmic trading systems as long as the drawdown experienced does not exceed their personal risk tolerance.

Final equity of a series of trades depends on position size.  Greater position size leads to greater final equity (up to a point that is not reached under ordinary trading circumstances), and also leads to greater drawdowns. 

There are two systems in play whenever a system is being traded.  The Trading System which processes the data, identifies patterns, and generates signals.  And the Trading Management System which processes recent trades and determines the health of the system.  

Position size is a parameter of the Trading Management System, not the Trading System.  Trading management monitors recent performance and adjusts position size appropriately.  If the system begins to break down, it is the trading management system that detects that problem and reduces the position size accordingly -- perhaps taking it offline completely.

The trading system has no way of knowing whether the recent trades are similar to those discovered during development or periods of earlier trading.  Moving position size to the trading system removes the only 'knob' the trader has to systematically respond to variation in the synchronization between the model and the data.  

## Establishing the parameters

The trader states his or her risk tolerance as follows:
'I am trading a $100,000 account and forecasting risk for the next two years.  I want to hold the risk of a drawdown from highest equity to date to a 5 percent chance of a 10 percent drawdown.'

There are four parameters in the statement:
1.  Account size:        100000.0
2.  Forecast horizon:    504       # 2 years of daily results.
3.  Drawdown tolerance:  0.10      # 10%
4.  Tail risk:           5         # the 95th percentile

Monte Carlo techniques are used to estimate the distribution of maximum drawdown and final equity with data drawn from a set of trades that represent the best estimate of future performance.

The best estimate set of trades can be any of:
* Real trades
* Paper trades
* Out-of-sample trades from development testing
* In-sample trades from development testing
* Hypothetical trades of interest

The most accurate results come when each trade is a one-day percentage change of the equity at risk that day.  A 2 year forecast horizon will have 504 single day trading results.

Multi-day trades can be used.  Compute the number of trades required to span a two year period.  Note that the intra-trade drawdown of multi-day trades will not be observed.  Conservatively and depending on the volatility of the issue being traded, the intra-trade drawdown will be greater than the closed trade drawdown by 2% for trades held an average of 5 days, 3% for trades held 20 days, and 5% for trades held 60 days.

## Getting Started

These programs were written in Python 3.7.

The example shows the procedures and a main program that calls them.

You will need the four procedures contained in the risk_normalization.py file.  Copy them into your Python program.

Your Python program will generate a list or numpy array of trades which will be passed as 'trades' to the risk_normalization procedure.

The examples included are each a single csv file containing one header line followed by one data value per row.  Several csv files have been included.  They are expected to be in the directory where your risk_normalization.py file is -- or adjust the path as necessary.  Set skiprows to 1 to account for the header. or set skiprows to 0 if the data you pass has no header.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

# Overview of the program

Begin with a set of trades. These are analyzed to compute safe-f, and are assumed to be the best estimate of future performance. This set does not change throughout the procedure.

The simulation begins by setting the fraction an initial value of 1.00. Create many equally likely equity curves, measure the maximum drawdown of each, keep them in a list. Treat the list of max drawdowns as a distribution and determine the maximum drawdown at the high risk tail -- probably at the 95th percentile. Compare the trader's personal risk tolerance with the tail risk of the distribution. If they are equal, the current value of the fraction is safe-f. If they are not equal, adjust the fraction and repeat.

safe-f has been established.  It is a fraction, typically between 0.50 and 1.00.  It is the position size that will maximize equity growth while holding the drawdown of the trading account to within the risk of the trader.

Using safe-f as the fraction, create many equally likely equity curves, measure the final equity, keep that in a list. Treat the list of final equity as a distribution and determine the equity at the 25th percentile. Convert the relative gain in equity to a compound annual rate of return. That value is car25.

Return safe-f and CAR25

## Interpretation

Fitting an algorithmic model to a set of data results in a set of relationships that are contained in the model.  When that model is supplied with a similar set of data, the model identifies patterns that precede profitable trades and informs the trader of the appropriate signals.  The system will be profitable providing the future resembles the past.  That is, the distribution of patterns and trades in the live data must come from the same distribution as was provided for the model fitting.  safe-f is the position size associated with that model and data.  As the data changes, the patterns and profitability will change.  Adding recent trade results to the 'best estimate set' of trades will allow the risk normalization procedure to recalibrate safe-f, which enables the trader to adjust position size.      

## Assumptions 

A trade list has been created by some process. It could be live trades, validation trades, in-sample trades, or hypothetical trades. Each of the trades represents the gain from the trade for a single day.  If the trader is trading today's MOC to tomorrow's MOC, the gain would be the change in price from today's close to tomorrow's close. A gain of 1% is represented as 0.0100. A day where the position is flat has a gain of 0.00 There are about 252 trades per year The account is marked to market daily. The account is managed daily. The trader is able and willing to change position daily.

## Use:

safe-f, CAR25 = risk_normalization(   
        trades,
        number_trades_in_forcast,
        number_days_in_forecast, 
        initial_capital, 
        tail_percentage, 
        drawdown_tolerance, 
        number_equity_in_CDF  )

Parameters: 
* trades: The set of trades to evaluate. Expecting a numpy array with one dimension. 
* number_trades_in_forecast: the number of trades needed to complete the forecast period.
* number_of_days_in_forecast: the number of trading days in the forecast periods. 
    Typically 504 for a 2 year forecast number_repetitions:
* initial_capital: initial amount in the trading account. Default = $100,000.00. 
* tail_percentage: The percentage at which to measure the tail risk. Default = 5.
* drawdown_tolerance: The traders drawdown tolerance. Expressed as a proportion of maximum equity to date. Default = 0.10 == a 10% drawdown. 
* number_equity_in_CDF: The number of equity curves used to compute a single CDF. Default = 1000

Returns: 
* safe-f: The fraction of the trading account that will be used for each trade. 
* CAR25: The compound annual rate of return for the given set of trades and position size.

# Definitions of program variables

* drawdown: list used to accumulate day by day drawdown max_drawdown maximum drawdown to date 
* equity: list used to accumulate day by day equity 
* max_equity: maximum equity to date
* file_name: name of csv or txt file containing trades fraction: during calculations, the then current estimate of position size, safe-f initial_capital: trading account allocated to this system in dollars Typically 100000.0
* number of replications of calculation of safe-f and CAR25. Typically 10. 
* number_sequences: number of equity sequences used to form the distribution. Typically 1000.
* number_trades_in_forecast: number of trades in each equity sequence.  Either:

    **Same as number_forecast_days if marking to market and trading daily. 

    **A smaller number if trade data represents multi-day holding.

* number_trades_in_best_est:  number of trades in best estimate set of trades -- read from file or drawn from known distribution

# Program to create normally distributed trades

make_trade_list.py 

This program creates a list of trades drawn from a Normal distribution, and writes them to disc in a csv file readable by risk_normalization.py and useful for testing.

