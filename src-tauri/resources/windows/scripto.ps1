$outputPath = "output.json"  # Path where the JSON will be saved
$directoryPath = "C:\Users\vince\Documents\tauri-action-test\src-tauri\resources\windows"  # Path to your directory containing the DLL files

# Create an empty hashtable for the JSON structure
$jsonData = @{ bundle = @{ resources = @{} } }

# Get all .dll files in the directory
Get-ChildItem -Path $directoryPath -Filter *.dll | ForEach-Object {
    $fileName = $_.Name
    $relativePath = "./resources/windows/$fileName"
    $jsonData.bundle.resources[$relativePath] = "./$fileName"
}

# Convert the hashtable to JSON and save it
$jsonString = $jsonData | ConvertTo-Json -Depth 10 -Compress
Set-Content -Path $outputPath -Value $jsonString

Write-Host "JSON file generated at: $outputPath"