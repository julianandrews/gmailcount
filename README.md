# gmailcount

gmailcount is a simple script to count the number of emails in your gmail
inbox. It's primary purpose is to allow status-bar programs like xmobar or
i3bar to poll your inbox without the need to store your password in plaintext.

## Installation

Build with

```
cargo build --release
```

## Usage

```
Script to count the number of emails in your gmail inbox

USAGE:
    gmailcount [OPTIONS] <EMAIL_ADDRESS> [SUBCOMMAND]

ARGS:
    <EMAIL_ADDRESS>    Email Address to check

OPTIONS:
    -h, --help                 Print help information
    -t, --timeout <TIMEOUT>    Request timeout in seconds
    -V, --version              Print version information

SUBCOMMANDS:
    delete-password    Delete the password for the provided email address from the secret store
    help               Print this message or the help of the given subcommand(s)
    set-password       Set the password for the provided email address in the secret store
```

Before you can use gmailcount in your status bar, you'll need to run it with
the `set-password` flag to set the password for your email address. Once you've
set your password it will be stored in your system keyring. Any program using
`gmailcount` will need to have access to your keyring.

When used with no flags, `gmailcount` will print the number of emails in your
inbox to stdout or nothing in case of failure.

## Security concerns

One of the main goals of `gmailcount` is to provide a minimum level of
security. To that end, all requests are sent via SSL, passwords are stored in
your system keyring (and are presumably encrypted if your system keyring is
worth anything), and the recommended use pattern is with app passwords on
accounts with two-factor authentication enabled. This allows you to keep your
password out of your dotfiles and encrypted, and to revoke your password in
case your system is compromised.

Obviously though any system that allows your computer to poll your email
without any human interaction isn't going to be ideal from a security
standpoint. `gmailcount` is only as secure as your system keyring, which
depending on how you use it and your configuration may not be very secure at
all. Certainly if you're using `gmailcount` in a status bar, any one who
manages to get access to your logged in user account will have access to your
email, and if your keyring is secured by an insufficiently strong password,
someone with access to your hard drive may be able to crack your keyring
password and access your gmail password.

Use `gmailcount` at your own risk! Still, it should be a lot more secure than a
system that just stores your password as plain text at least.

Sample xmobar script
--------------------

Here's an example of a script suitable for use with xmobar:

    #!/usr/bin/env sh

    url='https://mail.google.com'
    email='example@gmail.com'

    full_text=$(/path/to/gmailcount -t 1 "$email")
    full_text=${full_text:-?}

    case $full_text in
      ''|*[!0-9]*) color=\#FF0000 ;;
      0)           color=\#888888 ;;
      *)           color=\#00FF00 ;;
    esac

    echo "<action=\`xdg-open $url\`><fc=$color>✉ $full_text</fc></action>"

Sample i3blocks script
----------------------

Here's one suitable for use with i3blocks:


    #!/usr/bin/env sh

    url='https://mail.google.com'
    email='example@gmail.com'

    [ "$BLOCK_BUTTON" = 1 ] && xdg-open "$url"

    full_text=$(/path/to/gmailcount -t 1 "$email")
    full_text=${full_text:-?}

    case $full_text in
      ''|*[!0-9]*) color=\#FF0000 ;;
      0)           color=\#888888 ;;
      *)           color=\#00FF00 ;;
    esac

    echo "$full_text"
    echo "$short_text"
    echo "$color"

Sample Asynchrnous xmobar script
--------------------------------

Here's a somewhat more sophisticated script for xmobar which never blocks
waiting for the google servers. It works by writing the data asynchronously to
a temp file. The first argument to the script will set a timeout for writing
the data so that you can check gmail just before your status bar updates.
Something similar should work for i3blocks.

    #!/usr/bin/env sh

    STATUSFILE=/tmp/.gmail-status
    GMAILCOUNT=/path/to/gmailcount
    SLEEPTIME=${1:-0}
    EMAIL='example@gmail.com'
    URL='https://mail.google.com'

    echo_status() {
      echo "<action=\`xdg-open $URL\`><fc=$2><fn=1></fn> $1</fc></action>"
    }

    write_data() {
      sleep "$SLEEPTIME"
      full_text=$("$GMAILCOUNT" "$EMAIL")
      full_text=${full_text:-?}

      case $full_text in
        ''|*[!0-9]*) color=\#FF0000 ;;
        0)           color=\#888888 ;;
        *)           color=\#00FF00 ;;
      esac

      echo_status "$full_text" "$color" > "$STATUSFILE"
    }

    touch "$STATUSFILE"
    output=$(cat "$STATUSFILE")
    [ ! -z "$output" ] && echo "$output" || echo_status "?" \#880088
    > "$STATUSFILE"
    write_data &
