# archive.is api exploration

### trying to archive a site
clicking "save" button with a url in the bar calls the following api:

**Request URL**
```
  Request URL: https://archive.ph/submit/?submitid=XsyYbX9a8aiFaUmVbhCCFgl1x0ukSnnULKgw52ZfM6JsmspLFezx51rSKUJnnmqg&url=https%3A%2F%2Fwww.theinformation.com%2Farticles%2Falphabets-google-and-deepmind-pause-grudges-join-forces-to-chase-openai%3Futm_source%3Dti_app
  Request Method: GET
  Status Code: 302 
  Remote Address: 91.193.43.144:443
  Referrer Policy: strict-origin-when-cross-origin
```
**Payload (decoded):**
  - **submitid:** XsyYbX9a8aiFaUmVbhCCFgl1x0ukSnnULKgw52ZfM6JsmspLFezx51rSKUJnnmqg
  - **url:** https://www.theinformation.com/articles/alphabets-google-and-deepmind-pause-grudges-join-forces-to-chase-openai?utm_source=ti_app

**Payload (encoded):**
 - **submitid:** XsyYbX9a8aiFaUmVbhCCFgl1x0ukSnnULKgw52ZfM6JsmspLFezx51rSKUJnnmqg
 - **url:** https%3A%2F%2Fwww.theinformation.com%2Farticles%2Falphabets-google-and-deepmind-pause-grudges-join-forces-to-chase-openai%3Futm_source%3Dti_app

**Response Headers**
```
  cache-control: private, no-cache, no-store, must-revalidate, maxage=0
  content-length: 0
  date: Thu, 30 Mar 2023 20:02:58 GMT
  expires: Sat, 01 Jan 2000 00:00:00 GMT
  location: https://archive.ph/x2vQs
  pragma: no-cache
  server: nginx
  x-host: p-archiveweb31
```
**Request Headers**
```
  :authority: archive.ph
  :method: GET
  :path: /submit/?submitid=XsyYbX9a8aiFaUmVbhCCFgl1x0ukSnnULKgw52ZfM6JsmspLFezx51rSKUJnnmqg&url=https%3A%2F%2Fwww.theinformation.com%2Farticles%2Falphabets-google-and-deepmind-pause-grudges-join-forces-to-chase-openai%3Futm_source%3Dti_app
  :scheme: https
  accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
  accept-encoding: gzip, deflate, br
  accept-language: en,en-GB;q=0.9,en-US;q=0.8,en-CA;q=0.7,es-US;q=0.6,es;q=0.5,it-IT;q=0.4,it;q=0.3
  cookie: ga=GA1.2.661111166.1680206568
  referer: https://archive.ph/
  sec-ch-ua: "Google Chrome";v="111", "Not(A:Brand";v="8", "Chromium";v="111"
  sec-ch-ua-mobile: ?0
  sec-ch-ua-platform: "Linux"
  sec-fetch-dest: document
  sec-fetch-mode: navigate
  sec-fetch-site: same-origin
  sec-fetch-user: ?1
  upgrade-insecure-requests: 1
  user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36
```

afterwards, archive.is redirects the user to the url listed in the `location` field of the returned headers above.  This is the actual archive entry users will use.

### second run
running the save request a second time (this time without the `?utm_source` parameter in the requested url), we get:

**Request URL**
```
  Request URL: https://archive.is/submit/?submitid=ZZiwDA836TdRU7X0tKLjBaqeQRi6F%2Bae2rYWPATBD6BsmspLFezx51rSKUJnnmqg&url=https%3A%2F%2Fwww.theinformation.com%2Farticles%2Falphabets-google-and-deepmind-pause-grudges-join-forces-to-chase-openai
  Request Method: GET
  Status Code: 302 
  Remote Address: 91.193.43.144:443
  Referrer Policy: strict-origin-when-cross-origin
```
**Payload (decoded)**:
  - **submitid:** ZZiwDA836TdRU7X0tKLjBaqeQRi6F+ae2rYWPATBD6BsmspLFezx51rSKUJnnmqg
  - **url:** https://www.theinformation.com/articles/alphabets-google-and-deepmind-pause-grudges-join-forces-to-chase-openai

**Payload (encoded):**
  *- *submitid:** ZZiwDA836TdRU7X0tKLjBaqeQRi6F%2Bae2rYWPATBD6BsmspLFezx51rSKUJnnmqg
  - **url:** https%3A%2F%2Fwww.theinformation.com%2Farticles%2Falphabets-google-and-deepmind-pause-grudges-join-forces-to-chase-openai

**Response Headers**
```
  cache-control: private, no-cache, no-store, must-revalidate, maxage=0
  content-length: 0
  date: Thu, 30 Mar 2023 20:12:14 GMT
  expires: Sat, 01 Jan 2000 00:00:00 GMT
  location: https://archive.is/GkZPl
  pragma: no-cache
  server: nginx
  x-host: p-archiveweb31
```
**Request Headers**
```
  :authority: archive.is
  :method: GET
  :path: /submit/?submitid=ZZiwDA836TdRU7X0tKLjBaqeQRi6F%2Bae2rYWPATBD6BsmspLFezx51rSKUJnnmqg&url=https%3A%2F%2Fwww.theinformation.com%2Farticles%2Falphabets-google-and-deepmind-pause-grudges-join-forces-to-chase-openai
  :scheme: https
  accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
  accept-encoding: gzip, deflate, br
  accept-language: en,en-GB;q=0.9,en-US;q=0.8,en-CA;q=0.7,es-US;q=0.6,es;q=0.5,it-IT;q=0.4,it;q=0.3
  cookie: a=GA1.2.661111166.1680206997
  referer: https://archive.is/
  sec-ch-ua: "Google Chrome";v="111", "Not(A:Brand";v="8", "Chromium";v="111"
  sec-ch-ua-mobile: ?0
  sec-ch-ua-platform: "Linux"
  sec-fetch-dest: document
  sec-fetch-mode: navigate
  sec-fetch-site: same-origin
  sec-fetch-user: ?1
  upgrade-insecure-requests: 1
  user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36
```
### third run (on website that's never been archived)
**Request URL**
```
Request URL: https://archive.is/submit/?submitid=ly1WQYvPxnOtd%2Fd9OJdEu7b7QwwdcZq%2FvfxsGZh1LjBsmspLFezx51rSKUJnnmqg&url=https%3A%2F%2Fgithub.com%2Funsafeoats
Request Method: GET
Status Code: 200 
Remote Address: 91.193.43.144:443
Referrer Policy: strict-origin-when-cross-origin
```
**Response Headers**
```
  accept-ranges: bytes
  cache-control: private, no-cache, no-store, must-revalidate, maxage=0
  content-encoding: gzip
  content-length: 244
  content-type: text/html;charset=utf-8
  date: Thu, 30 Mar 2023 20:55:39 GMT
  expires: Sat, 01 Jan 2000 00:00:00 GMT
  pragma: no-cache
  refresh: 0;url=https://archive.is/wip/WJW3i
  server: nginx
  x-host: p-archiveweb31
```
**Request Headers**
```
  authority: archive.is
  :method: GET
  :path: /submit/?submitid=ly1WQYvPxnOtd%2Fd9OJdEu7b7QwwdcZq%2FvfxsGZh1LjBsmspLFezx51rSKUJnnmqg&url=https%3A%2F%2Fgithub.com%2Funsafeoats
  :scheme: https
  accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
  accept-encoding: gzip, deflate, br
  accept-language: en,en-GB;q=0.9,en-US;q=0.8,en-CA;q=0.7,es-US;q=0.6,es;q=0.5,it-IT;q=0.4,it;q=0.3
  cookie: ga=GA1.2.661111166.1680209721
  referer: https://archive.is/
  sec-ch-ua: "Google Chrome";v="111", "Not(A:Brand";v="8", "Chromium";v="111"
  sec-ch-ua-mobile: ?0
  sec-ch-ua-platform: "Linux"
  sec-fetch-dest: document
  sec-fetch-mode: navigate
  sec-fetch-site: same-origin
  sec-fetch-user: ?1
  upgrade-insecure-requests: 1
  user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36
```
### questions

1) where does the `?submitid` field in the `/submit` route come from?  is it just random or does it need to be deterministic?
    - answer found: this is a unique code generated every time someone opens the archive homepage.  if you hit `https://archive.is` with a get request, the html returned will have the following section:
      ```html
      <input type="hidden" name="submitid" value="HtIfTUPGq8UZJNXhb+0DdVFW+6OMcgLs2Ma5Ijrb/5BsmspLFezx51rSKUJnnmqg"/>
      ```
    -  the `submitid` can be extracted from here.

### general workflow for archiving site

1) send get request to `https://archive.is" and extract submitid value`
2) send get request to `https://archive.is/submit/?submitid={encoded submitid}&url={encoded url}`
3) check response headers to see if `location` or `refresh` is present
  - if `location` is present, extract `location` and return it's value
    * returned `status codes` observed in this situation have all been in range [302,] so far
  - if `location` is not present but `refresh` is (with pattern `0;url=https://archive.is/wip/{new archive identifier}`), extract the future url (`https://archive.is/{new archive identifier}`) and return it
    * return `status codes` observed in this situation have all been in range [200,] so far
    * can also monitor the `wip` version of the url found above and wait until it returns a `location` field before returning the url
