gmailcount
==========
gmailcount is a simple script to count the number of emails in your gmail
inbox. It's primary purpose is to allow status-bar programs like xmobar or
i3bar to poll your inbox without the need to store your password in plaintext.

Installation
------------
gmailcount requires python3 and the `keyring` and `requests` libraries.
Depending on your platform and keyring backend you may need other libraries.
For example, if you're using gnome-keyring as your system keyring, you'll need
the `secretstorage` and `python-dbus` packages. For some configurations the
`keyrings.alt` package may be useful. For many systems the keyring should work
out of the box with no extra configuration. For more information on keyring
configuration check out the python keyring
[documentation](https://pypi.python.org/pypi/keyring#configure-your-keyring-lib)
to get a sense of how to configure your keyring.

When I have time I'll write a setup script and get gmailcount onto pypi, but
for the moment it's self-serve!

Usage
-----
    usage: gmailcount [-h] [-s | -d | -p] [--debug] email_address

    Check gmail message count.

    positional arguments:
      email_address         email address to use

    optional arguments:
      -h, --help            show this help message and exit
      -s, --set-password    set the password for email_address
      -d, --delete-password
                            delete the password for email_address
      -p, --prompt          have gmail-count prompt you for your password
      --debug               print any exception traceback

Before you can use gmailcount in your status bar, you'll need to run it with
the `-s` flag to set the password for your email address. Once you've set your
password it will be stored in your system keyring. Any program using your
gmailcount will need to have access to your keyring. 

When used with no flags, gmailcount will print the number of emails in your
inbox to stdout or nothing in case of failure. All errors are printed to stderr
so as not to interfere with programs like xmobar.

Security Concerns
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

Sample xmobar Script
--------------------
Here's an example of a script suitable for use with xmobar. A very similar
script should work for other status bar programs.

    #!/usr/bin/env sh

    email='example@example.com'
    url='https://mail.google.com'
    full_text=$(/full/path/to/gmailcount "$email")
    full_text=${full_text:-?}

    case $full_text in
      ''|*[!0-9]*) color=\#FF0000 ;;
      0)           color=\#888888 ;;
      *)           color=\#00FF00 ;;
    esac

    echo "<action=\`xdg-open $url\`><fc=$color>✉ $full_text</fc></action>"
