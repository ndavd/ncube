import JSZip from 'jszip'
import { useCallback, useEffect, useState } from 'react'

declare global {
  interface Window {
    get_drag_drop_data: () => string | null
    export_to_data_file: (dimension: number, data: string) => void
  }
}

const export_to_data_file = (dimension: number, data: string) => {
  const fileName = `${dimension}cube-${Math.floor(
    new Date().getTime() / 1000,
  )}.data`
  const formattedData = JSON.stringify(JSON.parse(data), null, 2)
  const url = URL.createObjectURL(
    new Blob([formattedData], { type: 'application/json' }),
  )
  const anchor = document.createElement('a')
  anchor.href = url
  anchor.download = fileName
  document.body.appendChild(anchor)
  anchor.click()
  document.body.removeChild(anchor)
}

const LATEST_RELEASE_URL = 'https://github.com/ndavd/ncube/releases/latest'
const GITHUB_URL = 'https://github.com/ndavd/ncube'

enum STATE {
  FETCHING_BINARY,
  LOADING_APP,
  LOADED,
}

const zip = new JSZip()

export const Page = () => {
  const [appState, setAppState] = useState(STATE.FETCHING_BINARY)
  const [isHovering, setIsHovering] = useState(false)
  const [dragDropData, setDragDropData] = useState<string | null>(null)
  const [hasNotification, setHasNotification] = useState(false)

  const isLoading = appState == STATE.LOADING_APP
  const hasLoaded = appState == STATE.LOADED
  const setLoaded = () => setAppState(STATE.LOADED)
  const setLoading = () => setAppState(STATE.LOADING_APP)

  const handleFetch = useCallback(async () => {
    const res = await fetch('/latest-release', {
      headers: { Accept: 'application/octet-stream' },
    })
    const wasmZipBlob = await res.blob()
    setLoading()
    const wasmZip = await zip.loadAsync(wasmZipBlob)
    const js = await wasmZip.files['ncube.js'].async('text')
    const wasm = await wasmZip.files['ncube_bg.wasm'].async('uint8array')
    const jsURL = URL.createObjectURL(
      new Blob([js], { type: 'application/javascript' }),
    )
    const wasmURL = URL.createObjectURL(
      new Blob([wasm], { type: 'application/wasm' }),
    )
    const jsModule = await import(/* @vite-ignore */ jsURL)

    jsModule
      .default(wasmURL)
      .then(setLoaded)
      .catch((error: Error) => {
        if (
          error.message.startsWith(
            'Using exceptions for control flow, don\'t mind me. This isn\'t actually an error!',
          )
        ) {
          setLoaded()
          return
        }
        throw error
      })
  }, [])

  useEffect(() => {
    handleFetch()
  }, [handleFetch])

  useEffect(() => {
    if (!hasLoaded) return
    const timeout = setTimeout(() => {
      setHasNotification(true)
    }, 3000)
    return () => clearTimeout(timeout)
  }, [hasLoaded])

  useEffect(() => {
    window.export_to_data_file = export_to_data_file
    window.get_drag_drop_data = () => {
      const data = dragDropData
      if (data) setDragDropData(null)
      return dragDropData
    }
  }, [dragDropData])

  const handleDragOver = async (e: React.DragEvent<HTMLCanvasElement>) => {
    e.nativeEvent.preventDefault()
    setIsHovering(true)
  }

  const handleDragLeave = async (e: React.DragEvent<HTMLCanvasElement>) => {
    e.nativeEvent.preventDefault()
    setIsHovering(false)
  }

  const handleDrop = async (e: React.DragEvent<HTMLCanvasElement>) => {
    e.nativeEvent.preventDefault()
    setIsHovering(false)
    const dataFile = e.nativeEvent.dataTransfer?.files.item(0) ?? null
    if (!dataFile) return
    setDragDropData(await dataFile.text())
  }

  const renderOK = (text: string) => (
    <div className="inline text-green">{text}</div>
  )
  const renderWarning = (text: string) => (
    <div className="inline font-bold text-primary">{text}</div>
  )
  const renderLink = (href: string, content: string) => (
    <a href={href} rel="noreferrer" target="_blank" className="font-bold">
      {content}
    </a>
  )

  const renderNcube = () => (
    <canvas
      id="bevy"
      style={{ display: hasLoaded ? 'inline' : 'none' }}
      onContextMenu={(e) => e.preventDefault()}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
    />
  )

  const renderDrop = () => (
    <div className="fixed w-full text-center text-3xl text-green/40">
      -- DROP --
    </div>
  )

  const renderLoader = () => (
    <div className="mx-auto w-full max-w-sm px-1 text-lg">
      <br />
      <div>-- ncube --</div>
      <br />
      <div>Fetching latest release... {isLoading && renderOK('DONE')}</div>
      {isLoading && (
        <>
          <div>Entering n-dimensional space...</div>
          {renderWarning('Brace yourself.')}
        </>
      )}
    </div>
  )

  const renderNotification = () => (
    <div className="absolute bottom-1 right-1 w-full max-w-2xl animate-appear">
      <button
        onClick={() => setHasNotification(false)}
        className="ml-auto mr-0 block text-primary"
      >
        close
      </button>
      <div className="border-2 border-primary bg-bg/60 p-1 px-2 text-sm backdrop-blur-sm">
        <div>Hey there! Would you like to know more? 🤔</div>
        <div>
          This software is fully Open Source and{' '}
          {renderLink(GITHUB_URL, 'available on GitHub')}.
        </div>
        <div className="inline">
          For a boost in performance, try out the{' '}
          {renderLink(
            LATEST_RELEASE_URL,
            'binaries for Linux, Windows and MacOS',
          )}
          !
        </div>
      </div>
    </div>
  )

  return (
    <div className="h-screen w-screen">
      {hasNotification && renderNotification()}
      {isHovering && renderDrop()}
      {!hasLoaded && renderLoader()}
      {renderNcube()}
    </div>
  )
}
