import struct
from pathlib import Path
import OpenImageIO as oiio
px = [(100,200,300),(400,500,600),(700,800,900),(1023,0,512)]
for padding in (0,4,12):
    b = bytearray(1024)
    def u32(offset,value): struct.pack_into('>I',b,offset,value)
    u32(0,0x802A5FD7); u32(4,1024); u32(8,1024)
    b[24:28]=b'V4.5'; b[193]=3
    for channel in range(3):
        offset=196+channel*28
        b[offset+1]=channel+1; b[offset+2]=10
        u32(offset+4,2); u32(offset+8,2)
    b[681]=0x85; u32(684,padding)
    for row in range(2):
        for r,g,blue in px[row*2:row*2+2]:
            b.extend(struct.pack('>I',(r<<22)|(g<<12)|(blue<<2)))
        b.extend(bytes([0xab])*padding)
    u32(20,len(b))
    path=Path(f'/tmp/padding-{padding}.cin'); path.write_bytes(b)
    inp=oiio.ImageInput.open(str(path))
    assert inp, oiio.geterror()
    data=inp.read_image(format=oiio.UINT16)
    assert data is not None, inp.geterror()
    inp.close()
    # Recover ten-bit codes: independent readers may expand to sixteen bits differently.
    actual=(data.reshape(-1).astype('uint32') >> 6).tolist()
    expected=[v for pixel in px for v in pixel]
    print(padding, actual, flush=True)
    assert actual==expected, (padding,actual,expected)
print('Independent decoder preserved every pixel for padding 0, 4, 12')
