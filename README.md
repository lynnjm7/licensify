# Licensify
This is a personal tool that I built to generate a Free and Open Source Software
(FOSS) license file. The tool provides command line options for customizing the 
year, name, organization, and project information used when creating the license
text. These values can also be set and re-used from a configuration file saved
in the user's home directory.

# Installation
To install Licensify, you will need to have Rust installed. For more information 
please visit [the Rust installation page](https://www.rust-lang.org/en-US/install.html). 

After you have installed Rust (which should include the `cargo` utility), you 
can install licensify with the following command:

```bash
cargo install --git https://github.com/lynnjm7/licensify
```

The installation will take a little while to build the release version of Licensify.
Afterwords, you should have the latest build and everything should be ready to go!

To verify that licensify has been properly installed you should be able to run 

```bash
licensify -V
```

which should show the licensify version information. 

To complete the installation, you should run the setup command that is built
into the tool and follow the on-screen instructions for configuring the utility.
See the [Setup](#setup) section of this document.

# Usage
There are several different ways to use this utility. For instruction usage notes
use

```bash
licensify --help
```

# Setup
To setup this tool, simply run: 

```bash
licensify --init
```

As part of the setup process, you will be prompted to enter a series of default
values to use when generating licenses in the future.

**NOTE: You must run `--init` prior to using this tool!!**

# Listing available options
To list the available license templates that are installed, use

```bash
ignorify --list
```

It is relatively trivial to `grep` through this list to search for a specific 
license.

```bash 
ignorify --list | grep mit
```

# Generating a license
To generate a license file, provide the license template that you would like to
use and redirect the output of the program to your desired file. 

For instance,

```bash
licensify mit > LICENSE
```

would write the text of the MIT license to a file named LICENSE with the default
configuration values used.

# Licenses
The licenses used in this project are housed in the 
[licenses](https://github.com/lynnjm7/licenses) repository and installed to your 
local machine. Installing the licenses to your machine allows you to work 
offline, add your own licenses, or customize the existing licenses to your needs.

# License
This project is released under the MIT license. For more information, refer to
the LICENSE file in this repository.
