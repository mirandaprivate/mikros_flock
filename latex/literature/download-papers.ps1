param(
    [string]$ManifestPath = (Join-Path $PSScriptRoot 'manifest.csv')
)

$ErrorActionPreference = 'Stop'
$rows = Import-Csv -LiteralPath $ManifestPath
$failures = [System.Collections.Generic.List[string]]::new()
$downloaded = 0
$skipped = 0

foreach ($row in $rows) {
    $target = Join-Path $PSScriptRoot $row.local_file
    $parent = Split-Path -Parent $target
    New-Item -ItemType Directory -Path $parent -Force | Out-Null

    if (Test-Path -LiteralPath $target) {
        $bytes = [System.IO.File]::ReadAllBytes($target)
        if ($bytes.Length -ge 5 -and [System.Text.Encoding]::ASCII.GetString($bytes, 0, 5) -eq '%PDF-') {
            Write-Host "SKIP  $($row.local_file)"
            $skipped++
            continue
        }
        Remove-Item -LiteralPath $target -Force
    }

    $partial = "$target.part"
    if (Test-Path -LiteralPath $partial) {
        Remove-Item -LiteralPath $partial -Force
    }

    try {
        Write-Host "GET   $($row.source_url)"
        & curl.exe -L --fail --silent --show-error --retry 3 --retry-delay 2 $row.source_url -o $partial
        if ($LASTEXITCODE -ne 0) {
            throw "curl exited with code $LASTEXITCODE"
        }

        $bytes = [System.IO.File]::ReadAllBytes($partial)
        if ($bytes.Length -lt 10240) {
            throw "download is too small ($($bytes.Length) bytes)"
        }
        if ([System.Text.Encoding]::ASCII.GetString($bytes, 0, 5) -ne '%PDF-') {
            throw 'download does not begin with a PDF header'
        }

        Move-Item -LiteralPath $partial -Destination $target -Force
        $downloaded++
    }
    catch {
        if (Test-Path -LiteralPath $partial) {
            Remove-Item -LiteralPath $partial -Force
        }
        $failures.Add("$($row.venue) $($row.year): $($row.title) -- $($_.Exception.Message)")
        Write-Warning $failures[$failures.Count - 1]
    }
}

Write-Host "Downloaded: $downloaded; already present: $skipped; failed: $($failures.Count)"
if ($failures.Count -gt 0) {
    $failures | ForEach-Object { Write-Host "FAILED $_" }
    exit 1
}
