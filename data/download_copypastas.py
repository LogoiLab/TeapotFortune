#!/usr/bin/env python3

from bs4 import BeautifulSoup
from fake_useragent import UserAgent
import os
import requests
import sqlite3
from tqdm import tqdm

ua = UserAgent()
urls = open('./copypasta_urls.lst', 'r')
url_lines = urls.readlines()
dbcon = sqlite3.connect("copypastas.sqlite")
dbcur = dbcon.cursor()

lines_count = len(url_lines)
current_line = 0;

for url in tqdm(url_lines):
    headers = {
        'User-Agent': ua.random
    }
    url = url.strip()
    #print(url)
    response = requests.get(url, headers=headers)
    #print(response);
    soup = BeautifulSoup(response.content, "html.parser")
    copypasta_divs = soup.find_all("div", {"class": "copypasta"})
    if len(copypasta_divs) >= 1:
        data = [
            current_line,
            copypasta_divs[0].decode_contents()
        ]
        dbcur.execute("INSERT OR REPLACE INTO copypastas VALUES(?, ?)", data)
        current_line = current_line + 1
    if current_line % 100 == 0:
        dbcon.commit()
