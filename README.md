
# Table of Contents

1.  [What is this?](#org2f8e450)
2.  [Installation](#org3c5e5ed)
    1.  [AUR](#org145cbf0)
    2.  [crates.io (rusts package manager)](#orge68e905)
    3.  [From the release page](#org0575a07)
3.  [Usage:](#org1ebb9bb)
4.  [Security](#org0746e4d)

yas - a (kind of) sudo replacement
(entire readme is still work in progress)


<a id="org2f8e450"></a>

# What is this?

yas, or &rsquo;yet another sudo&rsquo;, is kind of a sudo replacement, written in rust and without really any configuration options or not needed features.
It is intended for single user systems only, as it works out of the box, with no configuration needed, but you also can&rsquo;t restrict a user from running stuff as root.
yas also can&rsquo;t run commands as any other user, but root.
In case you really do have a user, that shouldn&rsquo;t be allowed to run stuff as root, this isn&rsquo;t for you.


<a id="org3c5e5ed"></a>

# Installation


<a id="org145cbf0"></a>

## AUR

There is a AUR package for the git version of yas, you can install it with a AUR helper, or just git clone it manually:

    git clone ssh://aur@aur.archlinux.org/yas-git.git
    cd yas-git
    makepkg -is

or:

    yay -S yas-git


<a id="orge68e905"></a>

## crates.io (rusts package manager)

Get it from crates.io with rusts cargo, by running:

    cargo install yas

NOTE: The suid bit will not be set, you will manually have to set it for yas to work:

    su -c "chown root ~/.cargo/bin/yas && chmod u+s ~/.cargo/bin/yas"


<a id="org0575a07"></a>

## From the release page

1.  Download it
2.  Change the owner to root

    chmod root yas

1.  Set the suid bit

    chmod u+s yas

1.  Put it somewhere in your path, such as /usr/bin


<a id="org1ebb9bb"></a>

# Usage:

    yas - execute commands as the root user
    
    usage: yas [-h/--help] [-v/--version ]<command> <arguments for the command, this can be chained infinite>

As one can see, there aren&rsquo;t really any options for yas.
99% of the people who use sudo (or opendoas), just use it with default configuration and the default options, why also would you need a flag, to edit a file, if nobody ever uses that, and instead just runs \`sudo <editor> <file>\`


<a id="org0746e4d"></a>

# Security

yas will ask for the password whenever you want to run a command privileged.
Currently, there isn&rsquo;t any timeout programmed, but this feature is planned.

yas doesn&rsquo;t ship with so many security features, as sudo, so if you want the most secure root commmand executor, you should also use sudo

