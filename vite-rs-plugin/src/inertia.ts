/**
 * Helper function to resolve page components.
 * Forked from
 * {@link https://github.com/laravel/vite-plugin|Laravel's Vite Plugin}.
 */
export async function resolvePageComponent<T>(
  path: string | string[],
  pages: Record<string, Promise<T> | (() => Promise<T>)>,
): Promise<T> {
  for (const p of (Array.isArray(path)
    ? path
    : [path])) {
    const page = pages[p]

    if (typeof page === 'undefined') {
      continue
    }

    return typeof page === 'function'
      ? page()
      : page
  }

  throw new Error(`Page not found: ${path}`)
}
