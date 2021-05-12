export function string_or_empty(s: string | null | undefined): string {
  switch (s) {
    case null:
    case undefined:
      return ''
    default:
      return s
  }
}
