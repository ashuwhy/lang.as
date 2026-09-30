import sys
import os

# Ensure we can import the local package
sys.path.insert(0, os.getcwd())

try:
    from aslang import core
    print(f"Successfully imported aslang.core version: {core.VERSION}")
except ImportError as e:
    print(f"Failed to import aslang.core: {e}")
    sys.exit(1)

def test_run_code():
    source = 'print("Hello from Rust Core!"); let x = 10 + 20; print(x);'
    print(f"\nRunning source code:\n{source}")
    try:
        output = core.run_code(source)
        print(f"\nOutput:\n{output}")
        if "Hello from Rust Core!" in output and "30" in output:
            print("✅ Test Passed")
        else:
            print("❌ Test Failed: valid output not found")
    except Exception as e:
        print(f"❌ Execution failed: {e}")

if __name__ == "__main__":
    test_run_code()
