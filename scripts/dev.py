#!/bin/bash
import subprocess, sys, os, psutil


def kill_by_name(name):
    for proc in psutil.process_iter():
        if proc.name() == name:
            try:
                proc.kill()
            except:
                pass


kill_by_name("telegramr.exe")
kill_by_name("node.exe")

subprocess.Popen(
    "cargo run --features \"use-ws\"",
    stdout=subprocess.PIPE,
    stderr=None,
    shell=True)
p = subprocess.Popen(
    "yarn --cwd ui start", stdout=subprocess.PIPE, stderr=None, shell=True)
output = p.communicate()
