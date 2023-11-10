
#### Function Library ####

import os
import sys
import time
import datetime

# Function to label a file with the current date and time
def label_file():
    # Get the current date and time
    now = datetime.datetime.now()
    # Format the date and time
    date_time = now.strftime("%Y-%m-%d-%H-%M-%S")
    # Return the date and time
    return date_time

# Function to place a string in a new file in the ../Box directory with the current date and time
def new_zettle(string):
    # Get the current date and time
    date_time = label_file()
    # Create a new file in the ../Box directory with the current date and time
    file = open('Box/' + date_time + ".txt", "w")
    # Write the string to the file
    file.write(string)
    # Close the file
    file.close()