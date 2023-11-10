
## Nodule is a class that scans files in the Box directory
## and treats them as nodes in a graph.
## It looks for keywords in the files and uses them to
## create edges between the nodes.
## if a word exists in more than 5 % of the files, it is
## considered a stop word and is not used to create edges.

import os
import json
import re

class nodule:
    """Some Stuff"""

    def __init__(self, path):
        """Initialize the nodule with a path"""
        self.path = path
        self.nodes = {}
        self.edges = {}
        self.stop_words = []
        self.keywords = []

        #function which loads the current state of the nodule from a log json file
        
    def load(self):
        """Load the nodule from a log file"""
        pass

        #function which saves the current state of the nodule into a log json file   
    def save(self):
        """Save the nodule to a json log file"""
        pass

        #function which creates a list of nodes from the list of files
    def create_nodes(self):
        """Create a list of nodes from the list of files"""

        self.nodes = {}
        for file in os.listdir(self.path):
            with open(self.path + file, 'r') as f:
                data = f.read()
                if len(data) > 0:
                    self.nodes[file] = (data)


        #function which creates a list of stop words
    def create_stop_words(self):
        """Create a list of stop words which appear in more than 5% of the files"""
        pass

        #function which creates a list of keywords
    def create_keywords(self):
        """Create a list of keywords"""
        pass

        #function which creates a list of edges between the nodes
    def create_edges(self):
        """Create a list of edges between the nodes"""
        pass


    