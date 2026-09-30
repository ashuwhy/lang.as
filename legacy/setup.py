#!/usr/bin/env python3
# Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

from sys import argv
from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension
from setuptools.command.build_ext import build_ext
import subprocess
import os

def ensure_dir(path):
    if not os.path.exists(path):
        os.makedirs(path)

class CustomBuildExt(build_ext):
    def run(self):
        # Ensure directories exist
        for dir_path in [
            'bindings/rust/array_ops',
            'bindings/cpp',
            'bindings/wasm'
        ]:
            ensure_dir(dir_path)
            
        # Build Rust components (Standalone shared library for FFI)
        subprocess.check_call(['cargo', 'build', '--release', '-p', 'aslang', '--no-default-features'])
        
        # Build language-specific components
        try:
            # C++ components
            subprocess.check_call(['cmake', '.'], cwd='bindings/cpp')
            subprocess.check_call(['make'], cwd='bindings/cpp')
        except subprocess.CalledProcessError as e:
            print(f"Warning: Failed to build some components: {e}")
            print("Continuing with partial build...")
        
        super().run()

setup(
    name='aslang',
    version='0.1.0',
    description='A high-performance multi-language programming language',
    long_description=open('README.md').read(),
    long_description_content_type = "text/markdown",
    url = "https://github.com/alexshcer/aslang", 
    project_urls={
        "Documentation": "https://github.com/alexshcer/aslang/tree/main/docs",
        "Issue tracker": "https://github.com/alexshcer/aslang/issues",
    },
    author='Ashutosh Sharma',
    author_email='ashutoshsharmawhy@gmail.com',
    license='MIT', 
    classifiers=[
        'Development Status :: 4 - Beta',
        'Intended Audience :: Developers',
        'License :: OSI Approved :: MIT License',
        'Programming Language :: Python :: 3',
        'Programming Language :: Rust',
        'Programming Language :: C++',
        'Programming Language :: Go',
        'Programming Language :: Julia',
    ],
    keywords='aslang, programming language, multi-language, hybrid', 
    package_dir={'': 'src'},
    packages=find_packages(where='src'),
    install_requires=[
        'sly>=0.4',
        'setuptools-rust>=1.5.2',
    ],
    python_requires='>=3.6',
    rust_extensions=[
        RustExtension(
            "aslang.core",
            "Cargo.toml",
            binding=Binding.PyO3
        ),
    ],
    zip_safe=False,
    cmdclass={
        'build_ext': CustomBuildExt,
    },
    include_package_data=True,
)