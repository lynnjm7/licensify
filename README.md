# Licensify
This is a personal tool that I build to generate a Free and Open Source Software
(FOSS) license file. The tool provides command line options for customizing the 
year, name, organization, and project information used when creating the license
text. These values can also be set and re-used from a configuration file saved
in the user's home directory.

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
values to use when generating licenses.

# Listing available options
To list the available license templates that are installed, use

```bash
ignorify --list
```

It is relatively trivial to `grep` through this list to be able to search for a
specific license.

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

# License
This project is released under the MIT license. For more information, refer to
the LICENSE file in this repository.
