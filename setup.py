import os
from setuptools import setup


def read(fname):
    return open(os.path.join(os.path.dirname(__file__), fname)).read()


setup(
    name="gmailcount",
    version='0.1.4',
    author="Julian Andrews",
    author_email="jandrews271@gmail.com",
    description=("Script to count the number of emails in your gmail inbox"),
    license="MIT",
    keywords="email gmail script",
    url="http://packages.python.org/gmailcount",
    long_description=read('README.md'),
    classifiers=[
        "Development Status :: 4 - Beta",
        "Environment :: Console",
        "License :: OSI Approved :: MIT License",
        "Topic :: Communications :: Email",
        "Topic :: Utilities",
    ],
    scripts=['gmailcount'],
    install_requires=['requests>=2.9', 'keyring>=8.4'],
    extras_require={
        'secretservice': ['secretstorage'],
        'altkeyrings': ['keyrings.alt'],
    }
)
