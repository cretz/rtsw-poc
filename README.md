## Rust + Tor (embedded) + Static (compile) + Windows + Proof of Concept

This project makes a standalone executable built in Rust that statically compiles Tor and all of its dependencies. It
uses the new (as of this writing)
[0.3.3.1-alpha](https://blog.torproject.org/tor-0331-alpha-released-back-unstable-development) version which includes
embedding via a small, supported API. The simple goal of this project is to make the Tor version string appear. I put a
compiled version of `rtsw-poc.exe` in the [releases area](https://github.com/cretz/rtsw-poc/releases).

The [vendor/](vendor) folder contains all of the dependencies as submodules. So clone this repository with
`--recursive`. The dependencies are:

* [OpenSSL](https://github.com/openssl/openssl/) - Checked out at tag `OpenSSL_1_0_2n`
* [Libevent](https://github.com/libevent/libevent) - Checked out at tag `release-2.1.8-stable`
* [zlib](https://github.com/madler/zlib) - Checked out at tag `v1.2.11`
* [XZ Utils](https://git.tukaani.org/?p=xz.git) - Checked out at tag `v5.2.3`
* [Tor](https://github.com/torproject/tor) - Checked out at tag `tor-0.3.3.1-alpha`

Many many bugs and quirks were hit while deriving these steps. Also many other repos, mailing lists, etc were leveraged
to get some of the pieces right. They are not listed here for brevity reasons.

### Building

#### Msys2 and MinGW Setup

Tor is not really designed to work well with MSVC so we use MinGW instead. Since we are statically compiling, this means
we use the MinGW form of Rust too. In order to compile the dependencies, Msys2 + MinGW should be installed.

Download and install the latest [MSYS2 64-bit](http://www.msys2.org/) that uses the `MinGW-w64` toolchains. Once
installed, open the "MSYS MinGW 64-bit" shell link that was created. Once in the shell, run:

    pacman -Syuu

Terminate and restart the shell if asked. Rerun this command as many times as needed until it reports that everything is
up to date. Then in the same mingw-64 shell, run:

    pacman -Sy --needed base-devel mingw-w64-i686-toolchain mingw-w64-x86_64-toolchain \
                        git subversion mercurial \
                        mingw-w64-i686-cmake mingw-w64-x86_64-cmake

This will install all the tools needed for building and will take a while. Once complete, we have to downgrade a couple
of packages due to a bug in the current MinGW libraries. In the same shell, run:

    pacman -U /var/cache/pacman/pkg/mingw-w64-x86_64-crt-git-5.0.0.4745.d2384c2-1-any.pkg.tar.xz
    pacman -U /var/cache/pacman/pkg/mingw-w64-x86_64-headers-git-5.0.0.4747.0f8f626-1-any.pkg.tar.xz
    pacman -U /var/cache/pacman/pkg/mingw-w64-x86_64-winpthreads-git-5.0.0.4741.2c8939a-1-any.pkg.tar.xz \
              /var/cache/pacman/pkg/mingw-w64-x86_64-libwinpthread-git-5.0.0.4741.2c8939a-1-any.pkg.tar.xz

At least these were the cached package names on my install, they may be different on others. Once complete, MinGW is now
setup to build the dependencies.

#### Clone Repo

Inside the mingw-64 shell, clone this repo and submodules:

    git clone --recursive https://github.com/cretz/rtsw-poc.git

Then you can `cd rtsw-poc`. We will assume throughout this guide that you are starting at the cloned root.

#### OpenSSL

Inside the mingw-64 shell, navigate to the OpenSSL folder and build it:

    cd vendor/openssl
    ./Configure --prefix=$PWD/dist no-shared no-dso no-zlib mingw64
    make depend
    make
    make install

This will put OpenSSL libs at `dist/lib`.

#### Libevent

Inside the mingw-64 shell, navigate to the Libevent folder and build it:

    cd vendor/libevent
    ./autogen.sh
    ./configure --prefix=$PWD/dist --disable-shared --enable-static --with-pic
    make
    make install

This will put Libevent libs at `dist/lib`.

#### zlib

Inside the mingw-64 shell, navigate to the zlib folder and build it:

    cd vendor/zlib
    PREFIX=$PWD/dist make -fwin32/Makefile.gcc
    PREFIX=$PWD/dist BINARY_PATH=$PWD/dist/bin INCLUDE_PATH=$PWD/dist/include LIBRARY_PATH=$PWD/dist/lib make install -fwin32/Makefile.gcc

This will put zlib libs at `dist/lib`.

#### XZ Utils

Inside the mingw-64 shell, navigate to the XZ Utils folder and build it:

    cd vendor/xz
    ./autogen.sh
    ./configure --prefix=$PWD/dist \
                --disable-shared \
                --enable-static \
                --disable-doc \
                --disable-scripts \
                --disable-xz \
                --disable-xzdec \
                --disable-lzmadec \
                --disable-lzmainfo \
                --disable-lzma-links
    make
    make install

This will put XZ Utils libs at `dist/lib`.

#### Tor

Inside the mingw-64 shell, navigate to the tor folder and build it:

    cd vendor/tor
    ./autogen.sh
    LIBS=-lcrypt32 ./configure --prefix=$PWD/dist \
                                --disable-gcc-hardening \
                                --enable-static-tor \
                                --enable-static-libevent \
                                --with-libevent-dir=$PWD/../libevent/dist \
                                --enable-static-openssl \
                                --with-openssl-dir=$PWD/../openssl/dist \
                                --enable-static-zlib \
                                --with-zlib-dir=$PWD/../openssl/dist \
                                --disable-system-torrc \
                                --disable-asciidoc
    ln -s $PWD/../zlib/dist/lib/libz.a $PWD/../openssl/dist/lib/libz.a
    make
    make install

This will put Tor libs throughout the `src` area.

#### Rust

Install Rust in Windows which installs `rustup`. Then, from a `cmd` prompt with administrator privileges, run:

    rustup toolchain install stable-x86_64-pc-windows-gnu
    rustup default stable-gnu

Rust is now set to the MinGW 64 compiler by default.

#### Proof of Concept Program

From a `cmd` prompt with administrator privileges, navigate to repo root and run:

    cargo run

This will output:

    Tor version 0.3.3.1-alpha (git-de8bc9eed6eaadfc).

To generate a self-contained executable, run:

    cargo build --release

The executable will be at `target/release/rtsw-poc.exe`. My version is ~21MB.

This uses the new [tor_api.h](https://github.com/torproject/tor/blob/tor-0.3.3.1-alpha/src/or/tor_api.h) introduced
[here](https://trac.torproject.org/projects/tor/ticket/23684). Ideally we would make `build.rs` work with multiple
targets, automate the building of dependencies, etc but this is just a PoC.