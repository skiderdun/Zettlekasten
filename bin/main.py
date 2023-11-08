
# this is the main file for the project
# it will be used to run the program

import lib
import UI as UI
import tkinter as tk

# Create the root window
root = tk.Tk()

# Create the UI
ui = UI.UI(root)

# Run the UI
if __name__ == "__main__":
    root.mainloop()