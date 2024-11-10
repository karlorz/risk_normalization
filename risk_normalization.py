#  CAR25onCSV.py
#
#  A program to read a .CSV file that contains a list of trades
#    and computes the safe-f and CAR25 metrics that best
#    estimate the future performance of the system that produced
#    these gains and losses.

#  The risk_normalization library is managed by PyPi.
#  Before running this program, use pip to install risk_normalization
#  If you have set up a virtual environment, change directory to
#    that environment.
#  Then:
#    pip3 install risk-normalization


import math
import matplotlib as plt
import numpy as np
import pandas as pd
import random
#import risk_normalization
# import sklearn as skl
import statistics
# import statsmodels as st

#  These do not have a __version__ method
#print (f'math version:               {math.__version__}')
#print (f'random version:             {random.__version__}')
#print (f'risk_normalization version: {risk_normalization.__version__}')
#print (f'statistics version:         {statistics.__version__}')

#  These do
#print (f'matplotlib version:         {plt.__version__}')
#print (f'numpy version:              {np.__version__}')
#print (f'pandas version:             {pd.__version__}')
#print (f'scikit-learn version:       {skl.__version__}')
#print (f'statsmodels version:        {st.__version__}')

#-----------------------------------------------------
"""
risk_normalization.py

Created on Mon Feb 28 19:45:18 2022

@author: howard bandy

This file created on Tuesday, October 9, 2020
Modified on Friday, December 11, 2020
... separated risk_normalization and related procedures
... in preparation for publishing on PyPI
Modified on Monday, February 28, 2022
... added missing global value
... modified calling sequence


Risk normalization routines designed by Dr. Howard Bandy, 
Blue Owl Press.

License:  MIT

This technique was originally published in the book,
"Modeling Trading System Performance," in 2011, as an 
Excel Add-in.
Published again in the book "Quantitative Technical Analysis,"
in 2014, as a Python program.

#-----------------
Overview of the function risk_normalization:

Begin with a set of trades.  These are analyzed as is to compute
safe-f, and are assumed to be the best estimate of future 
performance.  This set does not change throughout the procedure.

The risk normalization consists of two phases:
1.  Compute the maximum fraction of the trading account
    that can be used to take a position in the tradable issue
    without exceeding the personal risk tolerance of the
    trader.  This is called "safe-f"
2.  Using safe-f as a position size, compute the profit
    potential for the forecast period.  Convert the gain
    to Compound Annual rate of Return, called "CAR25"

#--- Compute safe-f

Set the fraction an initial value of 1.00
    Create many equally likely equity curves,
        measure the maximum drawdown of each,
        keep them in a list.
    Treat the list of max drawdowns as a distribution
        and determine the maximum drawdown at the high
        risk tail -- probably at the 95th percentile.
    Compare the trader's personal risk tolerance with
        the tail risk of the distribution.
        If they are equal, the current value of the
        fraction is safe-f.
        If they are not equal, adjust the fraction and 
        repeat.

safe-f has been established.

#--- Compute CAR25

Using safe-f as the fraction
    Create many equally likely equity curves,
        measure the final equity,
        keep that in a list.
    Treat the list of final equity as a distribution
        and determine the equity at the 25th percentile.
    Convert the relative gain in equity to a 
        compound annual rate of return.
        That value is car25.

Return the mean and standard deviation of both safe-f and CAR25

#-----------------

Assumptions
  A trade list has been created by some process.
    It could be live trades, validation trades, in-sample trades,
    or hypothetical trades.
  Each trade represents the gain in equity of a single day,
    resulting from a trade on that day,
    such as the change in price from today's close to tomorrow's
    close.
A gain of 1% is represented as 0.0100
A day where the position is flat has a gain of 0.0000
There are about 252 trades per year
The account is marked to market daily.
The account is managed daily.
The trader is able and willing to change position daily.

Use:
  safe-f, CAR25 = risk_normalization(
                    trades,
                    number_days_in_forecast,
                    number_trades_in_forecast,
                    initial_capital,
                    tail_percentile,
                    drawdown_tolerance,
                    number_equity_in_CDF,
                    number_repetitions
                  )

Parameters:
  trades:  The set of trades to evaluate.
      Expecting a numpy array with one dimension.
  number_days_in_forecast:  the forecast period.
      Typical = 504 for a 2 year forecast
      Typical = 252 for a 1 year forecast
  number_trades_in_forecast:
      The number of trades to draw for each equity sequence.
      If each trade represents one daya of market-to-market result:
          If there is an entry for each and every day of the 
              trading, including entries of 0.0 for days not traded: 
              Typical = the same as number_days_in_forecast.
          If there are entries for only those days traded:
              Typical = the expected number of days with trades
                        in the forecast period.
      If each trade represents a multiday trade:
          Typical = the expected number of trades in the forecast period.
  initial_capital:  initial amount in the trading account.
      Typical = $100,000.00
  tail_percentile:  The percentile at which to measure the tail risk.  
      Typical = 5  => the 95th percentile of the CDF.
  drawdown_tolerance:  The traders drawdown tolerance.
      Expressed as a proportion of maximum equity to date.
      Typical = 0.10  A 10% drawdown.
  number_equity_in_CDF:  The number of equity curves used 
      to compute a single CDF.  
      Typical = 1000
  number_repetitions:  The number of replications of calculation of 
      safe-f and CAR25 to compute the mean and standard deviation.  
      Typical = 10

Returns:
  safe_f_mean:  The fraction of the trading account that will be
                used for each trade.
  safe_f_stdev:  standard deviation of safe_f calculations.     
  CAR25_mean:  The compound annual rate of return for the given
                 set of trades and position size.
  CAR25_stdev:  standard deviation of CAR25 calculations.     


definitions of variables

drawdown:                 list used to accumulate day by day drawdown
max_drawdown              maximum drawdown to date
equity:                   list used to accumulate day by day equity
max_equity:               maximum equity to date
file_name:                name of csv or txt file containing trades
fraction:                 during calculations, the then current estimate
                              of position size, safe-f

"""


def make_one_equity_sequence(
    trades,
    fraction,
    number_days_in_forecast,
    number_trades_in_forecast,
    initial_capital         ):

    """
    Given a set of trades, draw a random sequence of trades
    and form an equity sequence.
    
    Parameters:
    trades:           the set of trades to be analyzed
    fraction:         the proportion of the trading account
                      to be used for each trade.
    number_days_in_forecast:    Length of forecast in days.                 
    number_trades_in_forecast:  Length of forecast in trades.
    initial_capital:  Starting value of the trading account.
    
    Returns:  
    Two scalars:
    equity:  The equity at the end of the sequence in dollars.
    max_drawdown:  The maximum drawdown experienced in the sequence
            as a proportion of highest equity marked to market
            after each trade.
    """

    #  initialize sequence

    equity = initial_capital
    max_equity = equity
    drawdown = 0.0
    max_drawdown = 0.0

    #  form the equity curve to display, if desired
    daily_equity = np.zeros(number_days_in_forecast)

    #  form sequence

    for i in range(number_trades_in_forecast):
        trade_index = random.randint(0, len(trades) - 1)
        trade = trades[trade_index]
        trade_dollars = equity * fraction * trade
        equity = equity + trade_dollars
        daily_equity[i] = equity
        max_equity = max(equity, max_equity)
        drawdown = (max_equity - equity) / max_equity
        max_drawdown = max(drawdown, max_drawdown)
    #  if necessary, fill remaining days    
    for i in range(number_trades_in_forecast,number_days_in_forecast):
        daily_equity[i] = equity
   
#    plt.plot(daily_equity)
#    plt.show()

    return (equity, max_drawdown)


def analyze_distribution_of_drawdown(
    trades,
    fraction,
    number_days_in_forecast,
    number_trades_in_forecast,
    initial_capital,
    tail_percentile,
    number_equity_in_CDF   ):

    """
    Returns:
    tail_risk:  The maximum drawdown at the tail_percentile
                    of the distribution using the 
                    current value of the position size.
    """
    equity_list = []
    max_dd_list = []

    for i in range(number_equity_in_CDF):
        equity, max_drawdown = make_one_equity_sequence(
                                trades, 
                                fraction, 
                                number_days_in_forecast,
                                number_trades_in_forecast,
                                initial_capital)
        equity_list.append(equity)
        max_dd_list.append(max_drawdown)

    sorted_max_dd = np.sort(max_dd_list)
#    plt.plot(sorted_max_dd)
#    plt.show()
    tail_risk = np.percentile(sorted_max_dd, 100 - tail_percentile)

    return tail_risk


def form_distribution_of_equity(
    trades,
    fraction,
    number_days_in_forecast,
    number_trades_in_forecast,
    initial_capital,
    number_equity_in_CDF       ):
    
#    plt.hist(trades,bins=50)
#    plt.show()
    equity_list = []
    max_dd_list = []

    for i in range(number_equity_in_CDF):
        equity, max_drawdown = make_one_equity_sequence(
                                trades, 
                                fraction, 
                                number_days_in_forecast,
                                number_trades_in_forecast,
                                initial_capital)
        equity_list.append(equity)
        max_dd_list.append(max_drawdown)

    sorted_equity = np.sort(equity_list)
#    plt.plot(sorted_equity)
#    plt.show()

    return sorted_equity


def risk_normalization(
        trades, 
        number_days_in_forecast, 
        number_trades_in_forecast,
        initial_capital, 
        tail_percentile, 
        drawdown_tolerance, 
        number_equity_in_CDF,
        number_repetitions
        ):
    """
    Calculates the safe-f and CAR25 metrics for a given set of trades.

    Parameters:
        trades (numpy.ndarray): The set of trades to evaluate.
        number_days_in_forecast (int): The forecast period in days.
        number_trades_in_forecast (int): The number of trades to draw for each equity sequence.
        initial_capital (float): The initial amount in the trading account.
        tail_percentile (float): The percentile at which to measure the tail risk (e.g., 5 for the 95th percentile).
        drawdown_tolerance (float): The trader's drawdown tolerance as a proportion of maximum equity to date.
        number_equity_in_CDF (int): The number of equity curves used to compute a single CDF.
        number_repetitions (int): The number of replications to compute mean and standard deviation.

    Returns:
        tuple: A tuple containing:
            - safe_f_mean (float): Mean of calculated safe-f values.
            - safe_f_stdev (float): Standard deviation of safe-f calculations.
            - CAR25_mean (float): Mean of calculated CAR25 values.
            - CAR25_stdev (float): Standard deviation of CAR25 calculations.
    """

    safe_fs = []
    TWR25s = []
    CAR25s = []
    
    desired_accuracy = 0.003
    
    for rep in range(number_repetitions):
    
        #  Fraction is initially set to use all available funds
        #  It will be adjusted in response to the risk of drawdown.
        #  The final value of fraction is safe-f
        
        fraction = 1.0
        done = False
        while not done:
            # print(f"fraction this pass:  {fraction:0.3f}")
            tail_risk = analyze_distribution_of_drawdown(
                            trades, 
                            fraction,
                            number_days_in_forecast,
                            number_trades_in_forecast,
                            initial_capital,
                            tail_percentile,
                            number_equity_in_CDF)
        
            # print(f"tail_risk this pass: {tail_risk:0.3f}")
            if abs(tail_risk - drawdown_tolerance) < desired_accuracy:
                done = True
            else:
                fraction = fraction * drawdown_tolerance / tail_risk
        
        #  print(f'final value: safe_f: {fraction:0.3f}')
        
        #  Compute CAR25
        #  fraction == safe_f
        #  Compute CDF of equity
        #  TWR25 is 25th percentile
        #  CAR25 is 25th percentile
        
        CDF_equity = form_distribution_of_equity(
                         trades, 
                         fraction,
                         number_days_in_forecast,
                         number_trades_in_forecast,
                         initial_capital,
                         number_equity_in_CDF)
        
        TWR25 = np.percentile(CDF_equity, 25)
        # print(f'terminal wealth: {TWR25:9.0f}')
        
        CAR25 = 100.0 * (math.exp((252.0 / number_days_in_forecast) * 
                                 math.log(TWR25/initial_capital)) - 1.0)
        
        print(f'Compound Annual Return: {CAR25:0.3f}%')
    
        safe_fs.append(fraction)
        TWR25s.append(TWR25)
        CAR25s.append(CAR25)
    
    #  end of rep loop
       
    # print(safe_fs)
    # print(TWR25s)
    # print(CAR25s)
    
    # print (f'mean and standard deviation are based on {number_repetitions}'
    #        ' calculations')    
    safe_f_mean = statistics.mean(safe_fs)
    # print (f'safe_f_mean:   {safe_f_mean:0.3f}')
    if number_repetitions > 2:
        safe_f_stdev = statistics.stdev(safe_fs)
    #     print (f'safe_f_stdev:  {safe_f_stdev:0.3f}')
    else:
        safe_f_stdev = 0.0
    #     print ('standard deviation calculation is not meaningful')
    
    TWR25_mean = statistics.mean(TWR25s)
    # print (f'TWR25_mean:   {TWR25_mean:0.0f}')
    if number_repetitions > 2:
        TWR25_stdev = statistics.stdev(TWR25s)
    #     print (f'TWR25_stdev:  {TWR25_stdev:0.3f}')
    else:
        TWR25_stdev = 0.0
    #     print ('standard deviation calculation is not meaningful')
    
    CAR25_mean = statistics.mean(CAR25s)
    # print (f'CAR25_mean:   {CAR25_mean:0.3f}%')
    if number_repetitions > 2:
        CAR25_stdev = statistics.stdev(CAR25s)
    #     print (f'CAR25_stdev:  {CAR25_stdev:0.3f}%')
    else:
        CAR25_stdev = 0.0
    #     print ('standard deviation calculation is not meaningful')
    
    return (safe_f_mean, safe_f_stdev, CAR25_mean, CAR25_stdev)

#-----------------------------------------------------

newline = '\n'

#  Estimate safe-f and CAR25 for a given csv file

file_name = 'generated_normal_trades.csv'

#  This file covers about 28 years of daily trading.
#  It contains 1185 trades.
#  A 2 year forecast will have about 42 trades. 

#base_path_to_trades = "/home/howard/ML4TProjects/CAR25onCSV/data/"
base_path_to_trades = "./data/"
path_to_trades = base_path_to_trades + file_name 
print (f'{newline}The data file being processed is: {path_to_trades}')

trades = np.loadtxt(path_to_trades)
print (f'There are {len(trades)} marked-to-market daily trades in the file')
print ('Here are the first 10 trades:')
print (trades[0:10])

number_of_years_in_CSV = 28
average_trades_per_year = len(trades) / number_of_years_in_CSV
years_to_forecast = 2

# Calculate number of days and trades in forecast period
number_days_in_forecast = int(years_to_forecast * 252)  # Assuming 252 trading days per year
number_trades_in_forecast = int(average_trades_per_year * years_to_forecast)

initial_capital = 100000.0
tail_percentile = 5
drawdown_tolerance = 0.10
number_equity_in_CDF = 1000
number_repetitions = 5

safe_f_mean,safe_f_stdev,CAR25_mean,CAR25_stdev = risk_normalization(
    trades,
    number_days_in_forecast,
    number_trades_in_forecast,
    initial_capital,
    tail_percentile,
    drawdown_tolerance,
    number_equity_in_CDF,
    number_repetitions)


print (f'CAR25 mean:   {CAR25_mean:.2f}%')
print (f'CAR25 stdev:  {CAR25_stdev:.2f}')
print (f'safe-f mean:  {safe_f_mean:.2f}')
print (f'safe-f stdev: {safe_f_stdev:.2f}')







