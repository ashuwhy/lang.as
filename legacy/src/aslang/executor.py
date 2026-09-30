# Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

import sys
import os

try:
    from aslang import core
except ImportError:
    print("Error: Could not import 'aslang.core'. Please ensure the Rust extension is built and installed.")
    print("Try running: pip install .")
    sys.exit(1)

def evaluate(source):
    """
    Executes AS Lang source code using the Rust core runtime.
    """
    try:
        # Pass the source string directly to the Rust core
        return core.run_code(source)
    except RuntimeError as e:
        print(f"Runtime Error: {e}")
        return None

def main():
    if len(sys.argv) > 1:
        filename = sys.argv[1]
        if os.path.exists(filename):
            with open(filename, 'r') as f:
                source = f.read()
            evaluate(source)
        else:
            print(f"Error: File '{filename}' not found.")
    else:
        print(f"AS Lang {core.VERSION} (Python Wrapper)")
        print("Type 'exit' to quit.")
        while True:
            try:
                line = input("as > ")
                if line == "exit":
                    break
                evaluate(line)
            except EOFError:
                break
            except KeyboardInterrupt:
                print("\nKeyboardInterrupt")
                break

if __name__ == "__main__":
    main()
