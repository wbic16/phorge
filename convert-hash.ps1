# performs string manipulation to compress a SHA-512 hash into a 32-byte base-64 string.
param(
  [string] $number
)
if ($number.length -ne 128) {
  $stream = [System.IO.MemoryStream]::new()
  $writer = [System.IO.StreamWriter]::new($stream)
  $writer.write($number)
  $writer.Flush()
  $stream.Position = 0
  $number = (Get-FileHash -InputStream $stream -Algorithm SHA512).Hash.ToLower()
} else {
  $number = $number.ToLower()
}

$bytes = [System.Convert]::FromHexString($number)
[Convert]::ToBase64String($bytes)