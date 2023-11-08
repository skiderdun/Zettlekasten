
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
        self.root.title("File Labeler")
        self.root.geometry("600x400")
        self.root.resizable(False, False)
        self.root.configure(background="white")
    
