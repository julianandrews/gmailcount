gmailcount
==========
gmailcount is a simple script to count the number of emails in your gmail
inbox. It's primary purpose is to allow status-bar programs like xmobar or
i3bar to poll your inbox without the need to store your password in plaintext.

Installation
------------
gmailcount is tested on python 3.5. It may work on older versions of python 3.

Installation is simple:

    pip3 install gmailcount

with Secret Service support (gnome-keyring):

    pip3 install gmailcount[secretservice]

gnome-keyring support also requires the `dbus-python` package which can't be
installed via pip. It can be installed via system package manager (python3-dbus
on Debian) or from source.

If your system keyring isn't working you can install alternative keyring support:

    pip3 install gmailcount[altkeyrings]

Alternative keyring support includes keyring backends that should work on
almost any system, but may not be as secure. For more information on keyring
configuration check out the python keyring
[documentation](https://pypi.python.org/pypi/keyring#configure-your-keyring-lib).

Usage
-----
    usage: gmailcount [-h] [-s | -d | -p] [-t TIMEOUT] [--debug] email_address

    Check gmail message count.

    positional arguments:
      email_address         email address to use

    optional arguments:
      -h, --help            show this help message and exit
      -s, --set-password    set the password for email_address
      -d, --delete-password
                            delete the password for email_address
      -p, --prompt          have gmail-count prompt you for your password
      -t TIMEOUT, --timeout TIMEOUT
                            request timeout
      --debug               print any exception traceback

Before you can use gmailcount in your status bar, you'll need to run it with
the `-s` flag to set the password for your email address. Once you've set your
password it will be stored in your system keyring. Any program using your
gmailcount will need to have access to your keyring. 

When used with no flags, gmailcount will print the number of emails in your
inbox to stdout or nothing in case of failure.

Security concerns
-----------------
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
email, and if your keyring is secured by an insufficently strong password,
someone with access to your harddrive may be able to crack your keyring
password and access your gmail password. 

Use gmailcount at your own risk! Still, it should be a lot more secure than a
system that just stores your password as plain text at least.

Sample xmobar script
--------------------
Here's an example of a script suitable for use with xmobar:

    #!/usr/bin/env sh

    url='https://mail.google.com'
    email='example@gmail.com'

    full_text=$(/path/to/gmailcount -t 0.3 "$email")
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

    full_text=$(/path/to/gmailcount -t 0.3 "$email")
    full_text=${full_text:-?}

    case $full_text in
      ''|*[!0-9]*) color=\#FF0000 ;;
      0)           color=\#888888 ;;
      *)           color=\#00FF00 ;; 
    esac

    echo "$full_text"
    echo "$short_text"
    echo "$color"
