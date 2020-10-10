#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
make_trade_list.py

Creates a list of trades and writes them to disk
The file can be read by the risk_normalization program

Created on Tue Sep 15 19:06:42 2020

@author: howard bandy
"""

import matplotlib.pyplot as plt
import numpy as np

mean_gain = 0.001
std_dev_gain = 0.003
number_trades = 1000

def make_trade_list( number_trades=number_trades,
                     mean_gain = mean_gain,
                     std_dev_gain = std_dev_gain):
    
    trades = np.random.normal(mean_gain,
                              std_dev_gain,
                              number_trades)
    return (trades)

trades = make_trade_list(number_trades,mean_gain,std_dev_gain)
# print (trades)

file_name = 'generated_normal_trades.csv'
f = open(file_name, 'w')
s = f'Trades drawn from Normal distribution with mean: {mean_gain:0.3f}' 
s = s + f' and stdev: {std_dev_gain:0.3f}\n'
f.write(s)    
for i in range(number_trades):
    s = str(trades[i]) + '\n'
    f.write(s)
f.close()

sorted_trades = np.sort(trades)

y_pos = np.arange(len(trades))
plt.bar(y_pos,sorted_trades)
plt.show()

##  End ##

