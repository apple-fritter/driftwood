# driftwood
## Mission Statement

Driftwood is a refactor of complex IRC logs, providing an efficient, innovative, and insightful log analysis solution. Its goal is to transform IRC logs into a linguistics goldmine by offering a unified log format and unlocking a world of possibilities for building invaluable corpora. Driftwood aims to deliver the following key features:

- Lightning-Fast Regex Parsing: The log analysis process is optimized with powerful pattern matching using regular expressions (Regex), enabling efficient data extraction and manipulation.

- Elevated Readability: By adopting a unified log format, Driftwood enhances the readability of IRC logs, making them more accessible and easier to comprehend.

- Unmatched Portability: Driftwood facilitates the seamless transfer and migration of data between different IRC clients by providing a common log format, ensuring compatibility and smooth transitions.

By incorporating Driftwood into your log analysis workflow, you can unlock new insights and harness the full potential of IRC logs for linguistic analysis and beyond.

### Facilitate the porting of data between different IRC clients.
By having a common log format, it becomes easier to transfer or migrate data between different IRC clients, ensuring compatibility and seamless transition.

### Optimize efficiency by utilizing regular expressions (Regex).
Regular expressions are powerful pattern matching tools that can be used to extract and manipulate data. By incorporating regular expressions into the log format, you can enhance the efficiency of data processing and analysis.

---

## Using UNICODE

In the Driftwood format, a Unicode character is chosen for a standard field separator. The IRC standard does not specify any character for this purpose, nor does it implement any strategy to separate data present on a line entry.

To parse the log files, a unique and uncommon Unicode character like the hot beverage character (☕) is used as a distinctive delimiter that is unlikely to occur naturally within the log data. This choice helps with parsing the logs using Regex patterns.

When using Regex to parse the log files, you can use the Unicode character (☕) as the separator to split the log entries into individual fields. For example, you can use the Regex pattern `☕` to split the log entries based on the hot beverage character.

**IRC Log File Format:**

Columns are separated by the hot beverage Unicode character (☕).
- The first column (col1) is left blank. You can use `#` to ignore the line in the program of your choice.
- The second column (col2) represents the hour (HH).
- The third column (col3) represents the minutes (MM).
- The fourth column (col4) represents the seconds (SS).
- The fifth column (col5) contains the sender.
- The sixth column (col6) contains the message text.
- The seventh column (col7) is left blank.

### Example IRC log entry:
```
#☕12☕34☕56☕GitHubFAN23☕Hello, world!☕
```

---

## Directory Structure:
#### Organizational method explained

```
┌─ [Server1]
│   ├─ [Channel1]
│   │   ├─ [2023]
│   │   │   ├─ [04]
│   │   │   │   ├─ 01.txt
│   │   │   │   └─ 02.txt
│   │   │   │
│   │   │   └─ [05]
│   │   │       ├─ 01.txt
│   │   │       └─ 02.txt
│   │   │
│   │   └─ [2024]
│   │       └─ ...
│   │
│   ├─ [Channel2]
│   │   └─ ...
│   │
│   └─ [Query1]
│       └─ ...
│
└─ [Server2]
    └─ ...
```

#### Example Directory Structure:

```
┌─ [Freenode]
│   ├─ [#programming]
│   │   ├─ [2023]
│   │   │   ├─ [04]
│   │   │   │   ├─ 01.txt
│   │   │   │   └─ 02.txt
│   │   │   │
│   │   │   └─ [05]
│   │   │       ├─ 01.txt
│   │   │       └─ 02.txt
│   │   │
│   │   └─ [2024]
│   │       └─ ...
│   │
│   └─ [#general]
│       └─ ...
│
└─ [EFnet]
       └─ ...
```

In this example, we have two IRC servers, `Freenode` and `EFnet`. Within the `Freenode` server, we have two channels, `#programming` and `#general`. The log files for each channel are organized by year, month, and day. For instance, the log file `01.txt` under the directory `2023/04` would contain the IRC log entries for April 1st, 2023, for the `#programming` channel on the Freenode server.

You can adapt this structure and create the necessary directories and log files based on your specific server, channel, and date information to maintain an organized collection of IRC logs. You may decide on another character to separate the fields, but it is important to consider the impacts on regex parsing.

---

## Included Implementations

In this repository, there are included transcribing implementation examples in Rust. Feel free to explore each implementation directory and adapt them to suit your specific needs.

> Prior to the 11 June 2023 update, the example implementations were incomplete and contained erroneous code. Please report any errors you may experience by creating a pull request.

#### Examples include:
- [IRC-Cloud](https://github.com/apple-fritter/driftwood/tree/main/IRC-Cloud/): Provides support for transcribing IRC logs from the IRC-Cloud format.
- [mIRC](https://github.com/apple-fritter/driftwood/tree/main/mIRC/): Offers functionality for transcribing IRC logs from the mIRC format.
- [WeeChat](https://github.com/apple-fritter/driftwood/tree/main/WeeChat/): Includes support for transcribing IRC logs from the WeeChat format.
- [X-Chat](https://github.com/apple-fritter/driftwood/tree/main/XChat/): Enables transcribing IRC logs from the X-Chat format.
- [ZNC](https://github.com/apple-fritter/driftwood/tree/main/ZNC/): Supports transcribing IRC logs from the ZNC format.

---

## A Proof of Concept

[scrimshaw](https://github.com/apple-fritter/scrimshaw) stands as the pioneering proof of concept and scraping implementation of the driftwood standard; Create a quoteslist of any given user, from your driftwood formatted logs. Written in Rust.

---

## Prerequisites

Before using the included implementations, please ensure you have the following prerequisites:

- Actual log data and/or a supported IRC client to generate it.

- **Rust**: Make sure you have Rust installed on your system. You can download and install Rust from the official Rust website: [https://www.rust-lang.org](https://www.rust-lang.org).

- **Cargo**: Cargo is the package manager and build system for Rust. It is usually installed automatically with Rust. Ensure that Cargo is available in your system by running the command `cargo --version` in your terminal or command prompt.

> The majority of the included implementations are designed to work without any additional external resources. You should be able to get started with the implementations once you have Rust and Cargo set up on your system; Previously,the IRC-Cloud implementation assumed that the export was decompressed from the original zip format provided by the service. This approach reduced resource requirements at execution time. Users are welcome to adapt the code to reimplement that strategy by commenting out the lines of code that handle the zipped archive.

---

## Applications

Here's an example in Python demonstrating how you can split a log entry using the hot beverage character as the field separator using the `re` module:

```python
import re

log_entry = '#☕12☕34☕56☕GitHubFAN23☕Hello, world!☕'
fields = re.split('☕', log_entry)

print(fields)
```
In this example, the re.split() function is used with the pattern ☕ to split the log entry into fields. The resulting fields list contains the separated values, including the empty string at the beginning and end.

You can use this approach to parse and process IRC log entries in your preferred programming language by adapting the code to the corresponding regex functions or libraries available.

Feel free to experiment with different programming languages and regex libraries to achieve the desired parsing and analysis of the IRC logs.

---

## Considerations
While the proposed IRC log file format using a Unicode character as a field separator can be useful, it's important to consider some potential limitations:

- Unicode Support: Ensure that your parsing tools and programming languages fully support Unicode characters. While most modern programming languages handle Unicode well, older or less common tools may have limitations or encoding issues.

- Compatibility: The chosen Unicode character, such as the hot beverage character (☕), may not be compatible with all systems, platforms, or software. Some systems or tools might not render or handle the character properly, leading to parsing or display issues.

- File Size: The Unicode character used as a field separator could slightly increase the file size, especially if the log files contain a significant amount of data. While the impact is generally minimal, it's worth considering if storage or bandwidth constraints are a concern.

- Standard Compliance: The proposed format does not adhere to any existing IRC log file standards. While this may not be a concern for this specific use case, it's essential to be aware that the format may not be universally recognized or compatible with other IRC log processing tools or systems.

---

## Porting data back out

While this repository does not provide a specific method to port the data back out to another IRC client, users of this format are welcome to fork the repository and develop their own methodology. If you come up with an interesting approach, I'm open to considering and backporting those changes to this repository.

Feel free to explore different ways to export the IRC log data from this unified format to suit your specific requirements and integrate it into other IRC clients or tools. The flexibility of this format allows you to adapt and extend it as needed to meet your data migration or integration needs.

---

## <a id="irc-meta"></a>🤪 IRC Meta
### <a id="fritterz"></a> [@apple-fritter](https://github.com/apple-fritter)'s IRC Repositories:

---

#### Driftwood Suite of IRC Analytics
###### Driftwood utilities
- [driftwood](https://github.com/apple-fritter/driftwood): A unified IRC log format definition. (Rust)
- [flotsam](https://github.com/apple-fritter/flotsam): Aggregate a per-user metric of flagged contributions to any given user. (Rust)
- [jetsam](https://github.com/apple-fritter/jetsam): Flag lines of driftwood formatted IRC logs for sanitization, moderation, or further review. (Rust)
- [scrimshaw](https://github.com/apple-fritter/scrimshaw): Create a quoteslist of any given user, from your driftwood formatted logs. (Rust)

##### Driftwood native logging plugins
- [weechat.driftwood](https://github.com/apple-fritter/weechat.driftwood): Natively log WeeChat messages in the driftwood standard. (Python)

---

#### heX-Chat
- [xchat.channel-moderation](https://github.com/apple-fritter/xchat.channel-moderation): Moderate an IRC channel. (Python)
- [doppelganger](https://github.com/apple-fritter/doppelganger): X-Chat mIRC imposter. Fingerprint subversion. (Python bundle)

---

#### IRCcloud
- [irccloud-to-weechat](https://github.com/apple-fritter/irccloud-to-weechat): Convert IRC logs from IRCcloud format to Weechat format. (Rust)
- [irccloud-to-xchat](https://github.com/apple-fritter/irccloud-to-xchat): Convert IRC logs from IRCcloud format to XChat format. (Rust)

---

#### WeeChat
- [weechat.driftwood](https://github.com/apple-fritter/weechat.driftwood): Natively log WeeChat messages in the driftwood standard. (Python)
- [weechat.ban-evasion-detection](https://github.com/apple-fritter/weechat.ban-evasion-detection): Detect and prevent ban evasion. (Python)
- [weechat.typo-aggregator](https://github.com/apple-fritter/weechat.typo-aggregator): Record misspelled words in a TSV (tab-separated values) file. (Python)
- [weechat.whois-aggregator](https://github.com/apple-fritter/weechat.whois-aggregator): Aggregate whois data in a rolling CSV file. (Python)
- [weechat.youtube-info](https://github.com/apple-fritter/weechat.youtube-info): Deprecated. Extract video information from a YouTube URL and post it back to the channel. (Python)
- [weechat.youtube-api](https://github.com/apple-fritter/weechat.youtube-api): Extract video information from a YouTube URL and post it back to the channel. (Python)

---

### IRC usage considerations
When working with any project involving IRC (Internet Relay Chat), it's important to keep the following considerations in mind to ensure a positive and respectful environment for all participants.

#### Philosophy of Use
Tailor your project's behavior and responses to align with the expected norms and conventions of IRC. Take into account the preferences and expectations of IRC users, ensuring that your project provides a seamless and familiar experience within the IRC ecosystem.

#### Foster a Positive and Inclusive Environment
Respect and adhere to the guidelines and policies of the IRC platform you are using. Familiarize yourself with the platform's rules regarding script usage, automation, and acceptable behavior. Comply with the platform's Terms of Service, and be mindful of any limitations or restrictions imposed by the platform. Strive to create an inclusive and welcoming environment where all users can engage respectfully and comfortably.

#### Respect the Rights and Dignity of Other Users
Maintain a polite and courteous demeanor in all interactions. Uphold the fundamental principles of respect, avoiding engagement in illegal, inappropriate, or offensive behavior. This includes refraining from using derogatory or inflammatory language, sharing explicit, triggering, or offensive content, engaging in harassment, or launching personal attacks. Obtain explicit consent before interacting with other users or sending automated responses. Respect the privacy of other users and avoid invading their personal space without their permission.

#### Respect the IRC Community and Channels
Avoid disrupting the normal flow of conversation within IRC channels. Ensure that your project's actions and responses do not cause unnecessary disruptions or inconvenience to other users. Implement mechanisms to prevent spamming or flooding the channel with excessive or irrelevant messages. Handle errors gracefully, preventing unintended behavior or disruptions to the IRC platform or the experiences of other users.

#### Ensure Compatibility
Consider the potential variations in behavior across different IRC platforms and clients. While aiming for compatibility, be aware that certain functionalities may not be available or consistent across all platforms. Test your project on multiple IRC platforms and clients to ensure compatibility and provide the best possible experience for users.

---

## Contributing
Contributions are welcome! If you'd like to contribute, please follow these steps:
1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them.
4. Push your changes to your forked repository.
5. Submit a pull request to the main repository.

---

## [Disclaimer](DISCLAIMER)
**This software is provided "as is" and without warranty of any kind**, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

**The authors do not endorse or support any harmful or malicious activities** that may be carried out with the software. It is the user's responsibility to ensure that their use of the software complies with all applicable laws and regulations.

---

## License
This project is licensed under the [MIT License](LICENSE).
