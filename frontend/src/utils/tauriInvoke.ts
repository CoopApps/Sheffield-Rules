import { invoke as tauriInvoke } from '@tauri-apps/api/core'

/**
 * Safe wrapper for Tauri invoke that retries on failure
 * This is necessary in development mode where Tauri might not be immediately available
 */
export async function invoke<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T> {
  // Try up to 10 times with 500ms delay between attempts
  const maxRetries = 10
  let lastError: Error | null = null

  for (let attempt = 0; attempt < maxRetries; attempt++) {
    try {
      return await tauriInvoke<T>(command, args)
    } catch (e) {
      lastError = e as Error

      // Only retry on specific Tauri initialization errors
      const errorMsg = String(e)
      if (errorMsg.includes('undefined') || errorMsg.includes('__TAURI__')) {
        if (attempt < maxRetries - 1) {
          // Wait before retrying
          await new Promise(resolve => setTimeout(resolve, 500))
          continue
        }
      } else {
        // Other errors should be thrown immediately
        throw e
      }
    }
  }

  // All retries failed
  console.error(`Command "${command}" failed after ${maxRetries} attempts:`, lastError)
  throw lastError || new Error(`Failed to invoke command: ${command}`)
}
