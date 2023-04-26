# IRC.log-refactor

I wanted a way to create a unified log format that can be used as a transitional format when porting in/out data between IRC clients, which could also serve as a way to facilitate parsing logs using regular expressions (Regex).

## Using UNICODE
I choose to use a Unicode character like the hot beverage character (☕) as a field separator in this IRC log file format, as the IRC standard does not specify a specific character as a field separator in log files.

Typically, IRC logs are stored in plain text format, and the format for parsing them may vary depending on the specific implementation or tools being used.

By introducing a unique and uncommon Unicode character, such as the hot beverage character (☕), as the field separator, you can use it as a distinctive delimiter that is unlikely to occur naturally within the log data. This choice can help with parsing the logs using Regex patterns.

When using Regex to parse the log files, match the Unicode character (☕) as the separator to split the log entries into individual fields. For example, the Regex pattern `☕` to split the log entries based on the hot beverage character.

## IRC Log File Format:

Columns are separated by the hot beverage unicode character (☕).
- The first column (col1) is left blank. By using `#`, you may ignore the line in the program of your choice.
- The second column (col2) represents the hour (HH).
- The third column (col3) represents the minutes (MM).
- The fourth column (col4) represents the seconds (SS).
- The fifth column (col5) contains the sender.
- The sixth column (col6) contains the message text.
- The seventh column (col7) is left blank.

## Example IRC log entry:

```
#☕12☕34☕56☕Hello, world!☕
```

## Directory Structure:

```
- Server1
  - Channel1
    - 2023
      - 04
        - 01.txt
        - 02.txt
      - 05
        - 01.txt
        - 02.txt
    - 2024
      - ...
  - Channel2
    - ...
  - Query1
    - ...
- Server2
  - ...
```
Example Directory Structure:

```
- Freenode
  - #programming
    - 2023
      - 04
        - 01.txt
        - 02.txt
      - 05
        - 01.txt
        - 02.txt
    - 2024
      - ...
  - #general
    - ...
- EFnet
  - ...
```

In this example, we have two IRC servers, `Freenode` and `EFnet`. Within the `Freenode` server, we have two channels, `#programming` and `#general`. The log files for each channel are organized by year, month, and day. For instance, the log file `01.txt` under the directory `2023/04` would contain the IRC log entries for April 1st, 2023, for the `#programming` channel on the Freenode server.

You can adapt this structure and create the necessary directories and log files based on your specific server, channel, and date information to maintain an organized collection of IRC logs.

## Included implementations
In this repository, have included some basic transcribing programs, written in Rust, for the following IRC clients:
- [IRC-Cloud](https://github.com/apple-fritter/IRC.log-refactor/tree/main/IRC-Cloud/src)
- [MIRC](https://github.com/apple-fritter/IRC.log-refactor/tree/main/MIRC/src)
- [WeeChat](https://github.com/apple-fritter/IRC.log-refactor/tree/main/WeeChat/src)
- [X-Chat](https://github.com/apple-fritter/IRC.log-refactor/tree/main/XChat/src)
- [ZNC](https://github.com/apple-fritter/IRC.log-refactor/tree/main/ZNC/src)

## Applications
Here's a Python example demonstrating how you could split a log entry using the hot beverage character as the field separator using the `re` module:

```
import re

log_entry = '#☕12☕34☕56☕Hello, world!☕'
fields = re.split('☕', log_entry)

Output: ['', '12', '34', '56', 'Hello, world!', '']
print(fields)
```

In this example, the `re.split()` function is used with the pattern `☕` to split the log entry into fields. The resulting fields list contains the separated values, including the empty string at the beginning and end.

## Considerations
While the proposed IRC log file format using a Unicode character as a field separator can be useful, it's important to consider some potential limitations:

- Unicode Support: Ensure that your parsing tools and programming languages fully support Unicode characters. While most modern programming languages handle Unicode well, older or less common tools may have limitations or encoding issues.

- Compatibility: The chosen Unicode character, such as the hot beverage character (☕), may not be compatible with all systems, platforms, or software. Some systems or tools might not render or handle the character properly, leading to parsing or display issues.

- File Size: The Unicode character used as a field separator could slightly increase the file size, especially if the log files contain a significant amount of data. While the impact is generally minimal, it's worth considering if storage or bandwidth constraints are a concern.

- Standard Compliance: The proposed format does not adhere to any existing IRC log file standards. While this may not be a concern for this specific use case, it's essential to be aware that the format may not be universally recognized or compatible with other IRC log processing tools or systems.

## [Disclaimer](DISCLAIMER)
**This software is provided "as is" and without warranty of any kind**, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

**The authors do not endorse or support any harmful or malicious activities** that may be carried out with the software. It is the user's responsibility to ensure that their use of the software complies with all applicable laws and regulations.

## License

These files released under the [MIT License](LICENSE).
