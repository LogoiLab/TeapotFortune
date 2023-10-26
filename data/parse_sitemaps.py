#!/usr/bin/env python3

from bs4 import BeautifulSoup
import os

sitemap_dir = "./sitemaps/"

def get_urls_of_xml(xml_path):
    with open(xml_path, 'r') as handle:
        soup = BeautifulSoup(handle, features="xml")

        links_arr = []
        for link in soup.findAll('loc'):
            linkstr = link.getText('', True)
            links_arr.append(linkstr)

        return links_arr


with open('./copypasta_urls.lst', mode='wt', encoding='utf-8') as outfile:
    for sitemap in os.listdir(sitemap_dir):
        f = os.path.join(sitemap_dir, sitemap)
        if os.path.isfile(f):
            links = get_urls_of_xml(f)
            outfile.write('\n'.join(links))
