import { Buffer } from 'node:buffer'

import { renderPage } from 'vike/server'

export const fetchLatestRelease = async () => {
  const wasmZip = await fetch('https://github.com/ndavd/ncube/releases/latest/download/wasm.zip')
  const wasmZipBlob = await wasmZip.blob()
  return Buffer.from(await wasmZipBlob.arrayBuffer())
}

/**
 * @param {import('@vercel/node').VercelRequest} req
 * @param {import('@vercel/node').VercelResponse} res
 */
export default async function handler(req, res) {
  const { url } = req
  console.log('Request to url:', url)
  if (url === undefined) throw new Error('req.url is undefined')

  if (url == '/latest-release') {
    try {
      const releaseBuf = await fetchLatestRelease()
      res.statusCode = 200
      res.setHeader('Content-Type', 'application/octet-stream')
      res.setHeader('Content-Length', releaseBuf.byteLength)
      res.end(releaseBuf)
    } catch (e) {
      console.error(e)
      res.statusCode = 500
      res.end()
    }
    return
  }

  const pageContextInit = { urlOriginal: url }
  const pageContext = await renderPage(pageContextInit)
  const { httpResponse } = pageContext
  console.log('httpResponse', !!httpResponse)

  if (!httpResponse) {
    res.statusCode = 200
    res.end()
    return
  }

  const { body, statusCode, headers } = httpResponse
  res.statusCode = statusCode
  headers.forEach(([name, value]) => res.setHeader(name, value))
  res.end(body)
}
