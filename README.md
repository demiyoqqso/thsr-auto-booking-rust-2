# THSR Auto Booking (Rust)

Rust-only THSR booking client. It polls the booking flow, selects the first available train, and continues the booking flow.

## CAPTCHA on Railway

Railway containers do not have a desktop GUI, so the program must **not** call `open`, `xdg-open`, or a system image viewer. Instead, the program starts a tiny HTTP server and embeds the CAPTCHA image directly in a browser page.

1. Deploy this project to Railway.
2. In the Railway service: **Settings -> Networking -> Generate Domain**.
3. Put the generated URL into `THSR_PUBLIC_URL` in Railway Variables, for example:
   `https://your-service.up.railway.app`
4. Redeploy.
5. When a CAPTCHA is required, the log prints a URL like:
   `https://your-service.up.railway.app/captcha/<token>/`
6. Open that URL on your own computer, enter the CAPTCHA, and press **送出**.
7. The Rust process receives the code and continues the booking flow.

The CAPTCHA is intentionally left for manual entry; this project does not attempt to bypass it.

## Environment variables

See `.env.example`.

For a typical one-way Taipei -> Zuoying search on 2026/09/20 at 18:00, the time index must match the project's `TIME_TABLE`; do not assume the displayed time itself is the index.
