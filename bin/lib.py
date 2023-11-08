
#### Function Library ####

import os
import sys
import time
import datetime

# Function to label a file with the current date and time
def label_file(file_name):
    # Get current date and time
    now = datetime.datetime.now()
    # Format date and time
    date_time = now.strftime("%Y-%m-%d_%H-%M-%S")
    # Split file name into name and extension
    name, extension = os.path.splitext(file_name)
    # Create new file name
    new_file_name = name + "_" + date_time + extension
    # Return new file name
    return new_file_name

# Function to create a directory
def create_dir(dir_name):
    # Check if directory exists
    if not os.path.exists(dir_name):
        # Create directory
        os.makedirs(dir_name)

