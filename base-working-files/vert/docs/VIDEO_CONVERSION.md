## Video conversion

This file covers how video conversion works when using VERT.

On VERT, video uploads to a server for processing by default. This is because video conversion is hard to do in a browser as it uses a lot of resources, and will end up running very slowly (if it even works at all).

Our answer to this is [`vertd`](https://github.com/VERT-sh/vertd), which is a simple FFmpeg wrapper built in Rust. If you don't understand all that technical jargon, it basically allows you to convert videos using the full capacity of your computer, which results in much faster conversion. It runs on your computer (or a server somewhere, if you know what you're doing), and the VERT web interface reaches out to it in order to convert your videos.

We host an official instance of [`vertd`](https://github.com/VERT-sh/vertd) so you do not have to host it yourself for convenience, but considering you're here, you probably want to host it for yourself. Essentially:

- Download the latest release of `vertd` for your machine [here](https://github.com/VERT-sh/vertd/releases)
- Run the server
- Connect the VERT UI to your local `vertd` instance by entering its IP & port
    - By default, `vertd` runs a HTTP server on port `24153`, so you would put `http://localhost:24153` in the "Instance URL" setting found in VERT's settings (assuming you are running it on your own PC)
