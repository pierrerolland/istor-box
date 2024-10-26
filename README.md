# ISTOR BOX

The box that tells stories by scanning RFID cards.

Works with a RC522 RFID card on a Raspberry (standard or Zero)

![RC522](https://m.media-amazon.com/images/I/51IjYWCcV7L._AC_UL320_.jpg)

--- 

## Software requirements

- Raspberry Pi OS
- Enable SPI interface in `raspi-config > Interfacing options`
- Enable Wi-fi
- Cargo

```bash
curl https://sh.rustup.rs -sSf | sh
```

## Lib requirements

```bash
libasound2-dev
libssl-dev
```

---

## Remote requirements

An API that can handle the following request:

```
GET BASE_URL/api/story/{id}
```

Where `id` is an RFID card UID (XXX.XXX.XXX.XXX)

This endpoint should return the MP3 contents directly.

---

## Environment variables to set

| variable          | value                                                        |
|-------------------|--------------------------------------------------------------|
| API_URL           | The URL of the Istor API where to fetch the stories from     |
| STORIES_DIRECTORY | Path of the local directory where the stories are downloaded |

---

## Installing

### Build the binary

```bash
cargo build --release
sudo mv ./target/istor-box /usr/local/bin/
```

### Setup the USB device (optional)

1. Run `sudo aplay -l` to ensure the USB device is connected
2. Create and open `/etc/modprobe.d/local.conf`
3. Add the following lines in it:
   ```bash
   options snd_usb_audio index=0
   options snd_bcm2835 index=1

   options snd slots=snd_usb_audio,snd_bcm2835,snd_hdmi
   ```
4. Open `~/.asoundrc` and put this content inside:
   ```
   pcm.!default {
       type hw
       card 0
   }

   pcm.output {
       type hw
       card 0
   }

   ctl.!default {
       type hw
       card 0
   }
   ```
 5. Copy it to `/root`
 6. Reboot
 7. Run `alsamixer` to adapt the volume

## Run at startup

Edit `/etc/rc.local` and add, before `exit 0`, the following line:

`sudo API_URL=<the api url> STORIES_DIRECTORY=<the local stories path> /usr/local/bin/istor-box &`

---

## Wiring

| Card | Pin | GPIO |
|------|-----|------|
| 3.3V | 1   |      |
| RST  | 22  | 25   |
| GND  | 6   |      |
| MISO | 21  | 09   |
| MOSI | 19  | 10   |
| SCK  | 23  | 11   |
| SDA  | 15  | 22   |

---

## Usage

Scan the cards and enjoy. First scan will download the MP3 from the API, subsequent ones
will use the downloaded contents directly.
