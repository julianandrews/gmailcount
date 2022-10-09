# gmailcount

gmailcount is a simple tool to count the number of emails in your gmail
inbox. It's primary purpose is to allow status-bar programs like xmobar or
i3bar to poll your inbox without the need to store your password in plaintext.

## Installation

Look for releases on github or build with

    cargo build --release

## Usage

You'll first need to generate a Google app password and store it in the system
keyring with:

    gmailcount your-email@gmail.com set-password

Then, to get a count of emails, simply run:

    gmailcount your-email@gmail.com

Any program running gmailcount will need to have access to your keyring. The
current count of unread emails in your inbox will print to stdout. In case of
error, all output will print to stderr.

See `gmailcount -h` for detailed usage.

## Security concerns

One of the main goals of gmailcount is to provide a minimum level of
security. To that end, all requests are sent via SSL, passwords are stored in
your system keyring (and are presumably encrypted if your system keyring is
worth anything), and the recommended use pattern is with app passwords on
accounts with two-factor authentication enabled. This allows you to keep your
password out of your dotfiles and encrypted, and to revoke your password in
case your system is compromised.

Obviously though any system that allows your computer to poll your email
without any human interaction isn't going to be ideal from a security
standpoint. gmailcount is only as secure as your system keyring, which
depending on how you use it and your configuration may not be very secure at
all. Certainly if you're using gmailcount in a status bar, any one who
manages to get access to your logged in user account will have access to your
email, and if your keyring is secured by an insufficiently strong password,
someone with access to your hard drive may be able to crack your keyring
password and access your gmail password.

Use gmailcount at your own risk! Still, it should be a lot more secure than a
system that just stores your password as plain text at least.

## Examples

### Simple xmobar script

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

### Simple i3blocks script

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

### Asynchronous example

The above scripts should work if your latency to google is low, but don't
behave particularly well if network connectivity is slow or interrupted.

Here's how I use gmailcount:

Systemd service (~/.config/systemd/<user>/gmailcount.service):

    [Unit]
    Description=Gmail Count Service

    [Service]
    ExecStart=/usr/bin/gmailcount <your-email>@gmail.com daemon --cache-dir /home/<user>/.cache/gmailcount --poll-frequency 10

    [Install]
    WantedBy=multi-user.target

Sample xmobar script:

    #!/usr/bin/env sh

    EMAIL="$1"
    URL='https://mail.google.com'
    CACHE_FILE="${XDG_CACHE_HOME:-$HOME/.cache}/gmailcount/$EMAIL"

    echo_status() {
      echo "<action=\`xdg-open $URL\`><fc=$2><fn=1></fn> $1</fc></action>"
    }

    get_status_output() {
      full_text=$(cat "$CACHE_FILE")
      full_text=${full_text:-?}

      case $full_text in
        ''|*[!0-9]*) color=\#dc322f ;;
        0)           color=\#586e75 ;;
        *)           color=\#2AA198 ;;
      esac

      echo_status "$full_text" "$color"
    }

    get_status_output
