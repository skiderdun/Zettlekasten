
### This is the UI class ###

import tkinter as tk
from tkinter import ttk
from tkinter import messagebox
from tkinter import filedialog
from tkinter import font

import os
import lib 

class UI:
#### Create and Manage the UI ####
    def __init__(self, root):
        self.root = root
        self.root.title("=Zettlekasten=")
        self.root.geometry("600x400")
        self.root.resizable(False, False)
        self.root.configure(background="grey")

        # Create a dynamic text box in the top center of the window
        self.text_box = tk.Text(self.root, height=1, width=50, font=("Helvetica", 16))
        self.text_box.pack(side=tk.TOP, pady=10, padx=10)

        # Create a button to submit the text in the text box as a new zettle
        self.submit_button = tk.Button(self.root, text="Submit", command=self.submit)
        self.submit_button.pack(side=tk.TOP, pady=10, padx=10)

    
    def submit(self):
        # Get the text from the text box
        text = self.text_box.get("1.0", "end-1c")
        # Create a new zettle with the text
        lib.new_zettle(text)
        # Clear the text box
        self.text_box.delete("1.0", "end-1c")

