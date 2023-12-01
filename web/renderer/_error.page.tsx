export const Page = ({ is404 }: { is404: boolean }) => {
  const msg = is404 ? '404 Page Not Found' : '500 Internal Error'
  return (
    <div className='flex h-screen items-center justify-center text-xl'>{msg}</div>
  )
}
