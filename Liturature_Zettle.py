### create a citation file for a given paper based on DOI
### usage: doi2bib.py <doi>

import sys 
import requests
import json
import re

def doi2bib(doi):
    url = "http://dx.doi.org/" + doi
    headers = {"accept": "application/x-bibtex"}
    r = requests.get(url, headers=headers)
    return r.text