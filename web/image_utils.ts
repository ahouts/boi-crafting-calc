import RequireContext = __WebpackModuleApi.RequireContext

export function import_all(r: RequireContext): Record<string, string> {
  const images: Record<string, string> = {}
  r.keys().map((item) => {
    images[item.replace('./', '')] = r(item)
  })
  return images
}
