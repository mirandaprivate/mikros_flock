from pathlib import Path

from pypdf import PdfReader


pdf_path = Path(r"C:\Users\miranda\Desktop\paper\packedSumcheck.pdf")
output_path = Path(__file__).with_name("packedSumcheck.txt")

reader = PdfReader(pdf_path)
with output_path.open("w", encoding="utf-8") as output:
    for page_number, page in enumerate(reader.pages, start=1):
        output.write(f"\n\n===== PDF PAGE {page_number} =====\n\n")
        output.write(page.extract_text() or "")

print(f"{len(reader.pages)} pages -> {output_path}")
