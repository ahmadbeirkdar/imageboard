import os
import sys
from google.cloud import vision
os.environ['GOOGLE_APPLICATION_CREDENTIALS'] = sys.argv[2]
# Instantiates a client
client = vision.ImageAnnotatorClient()

def label_image(file_name : str) -> str:

    with open(file_name, 'rb') as image_file:
        content = image_file.read()

    image = vision.Image(content=content)
    response = client.label_detection(image=image)
    labels = response.label_annotations

    ans = [i.description for i in labels if i.score > 0.5]

    return '|'.join(ans)

print(label_image(sys.argv[1]))
