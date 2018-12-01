#!/usr/bin/env python

"""
Run this in one of the puzzle directories (01, 02, etc).
"""

import os
import subprocess


if __name__ == "__main__":
    # Build Rust solution.
    if subprocess.call("cd solve; cargo build", shell=True) != 0:
        raise ValueError("cargo build returned non-zero exit status")

    # Check any available test cases.
    for input_path in os.listdir("."):
        if not input_path.startswith("test_input_"):
            continue
        print "Checking {}".format(input_path)
        correct_output_path = input_path.replace("input", "output")

        # Check Python solution.
        py_output_path = correct_output_path + "-py"
        if subprocess.call("python solve.py <{} >{}".format(
                input_path, py_output_path), shell=True) != 0:
            raise ValueError("solve.py returned non-zero exit status")
        if subprocess.call("diff {} {}".format(
                correct_output_path, py_output_path), shell=True) != 0:
            raise ValueError("solve.py returned a different solution")
        print "  Python: OK"

        # Check Rust solution.
        rs_output_path = correct_output_path + "-rs"
        if subprocess.call("./solve/target/debug/solve <{} >{}".format(
                input_path, rs_output_path), shell=True) != 0:
            raise ValueError("rust solve returned non-zero exit status")
        if subprocess.call("diff {} {}".format(
                correct_output_path, rs_output_path), shell=True) != 0:
            raise ValueError("rust solve returned a different solution")
        print "  Rust: OK"
