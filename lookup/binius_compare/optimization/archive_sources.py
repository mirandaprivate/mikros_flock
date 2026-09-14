"""Keep the exact measured optimization source version beside its results."""
from pathlib import Path
import hashlib
import json
import sys
import zipfile

def archive(folder):
    here=Path(__file__).resolve().parent
    metadata=json.loads((folder/'environment.json').read_text())
    with zipfile.ZipFile(folder/'sources.zip','x',compression=zipfile.ZIP_DEFLATED) as out:
        for name,expected in metadata['sources_sha256'].items():
            data=(here/name).read_bytes()
            assert hashlib.sha256(data).hexdigest()==expected,name
            out.writestr(name,data)
        out.writestr('environment.json',json.dumps(metadata,indent=2)+'\n')

if __name__=='__main__':
    archive(Path(sys.argv[1]))
