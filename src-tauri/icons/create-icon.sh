#!/bin/bash
# Create a simple placeholder icon for development
# This creates a 512x512 PNG with ImageMagick or Python PIL
if command -v convert &> /dev/null; then
    # Use ImageMagick
    convert -size 512x512 xc:#2196F3 -font Arial -pointsize 120 -fill white -gravity center -annotate +0+0 "SD" icon.png
    echo "✅ Icon created with ImageMagick"
elif python3 -c "from PIL import Image, ImageDraw, ImageFont" 2>/dev/null; then
    # Use Python PIL
    python3 << 'PYEOF'
from PIL import Image, ImageDraw, ImageFont
img = Image.new('RGB', (512, 512), color='#2196F3')
d = ImageDraw.Draw(img)
try:
    font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 180)
except:
    font = ImageFont.load_default()
d.text((256, 256), "SD", fill='white', anchor="mm", font=font)
img.save('icon.png')
print("✅ Icon created with Python PIL")
PYEOF
else
    # Fallback: create minimal 1x1 PNG (base64 encoded)
    echo "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==" | base64 -d > icon.png
    echo "✅ Created minimal placeholder icon"
fi
