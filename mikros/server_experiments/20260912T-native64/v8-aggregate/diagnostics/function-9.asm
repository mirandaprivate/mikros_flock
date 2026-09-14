000000000067fca0 <_RNvNtNtNtCs5Palesvl9wF_6mikros10univariate5tiled8affine_h16consume_h_affine>:
  67fca0:	48 81 ec 88 05 00 00 	sub    rsp,0x588
  67fca7:	89 d0                	mov    eax,edx
  67fca9:	83 e0 78             	and    eax,0x78
  67fcac:	49 89 d1             	mov    r9,rdx
  67fcaf:	49 c1 e9 03          	shr    r9,0x3
  67fcb3:	0f 84 24 08 00 00    	je     6804dd <_RNvNtNtNtCs5Palesvl9wF_6mikros10univariate5tiled8affine_h16consume_h_affine+0x83d>
  67fcb9:	62 f1 7c 48 10 07    	vmovups zmm0,ZMMWORD PTR [rdi]
  67fcbf:	62 f1 7c 48 11 04 24 	vmovups ZMMWORD PTR [rsp],zmm0
  67fcc6:	62 f1 7c 48 10 47 01 	vmovups zmm0,ZMMWORD PTR [rdi+0x40]
  67fccd:	62 f1 7c 48 11 44 24 	vmovups ZMMWORD PTR [rsp+0x40],zmm0
  67fcd4:	01 
  67fcd5:	62 61 fe 48 6f 7f 04 	vmovdqu64 zmm31,ZMMWORD PTR [rdi+0x100]
  67fcdc:	62 61 fe 48 6f 77 05 	vmovdqu64 zmm30,ZMMWORD PTR [rdi+0x140]
  67fce3:	62 f1 7c 48 10 47 08 	vmovups zmm0,ZMMWORD PTR [rdi+0x200]
  67fcea:	62 f1 7c 48 11 44 24 	vmovups ZMMWORD PTR [rsp+0x240],zmm0
  67fcf1:	09 
  67fcf2:	62 61 fe 48 6f 67 09 	vmovdqu64 zmm28,ZMMWORD PTR [rdi+0x240]
  67fcf9:	62 f1 7c 48 10 47 0c 	vmovups zmm0,ZMMWORD PTR [rdi+0x300]
  67fd00:	62 f1 7c 48 11 44 24 	vmovups ZMMWORD PTR [rsp+0x180],zmm0
  67fd07:	06 
  67fd08:	62 61 fe 48 6f 57 0d 	vmovdqu64 zmm26,ZMMWORD PTR [rdi+0x340]
  67fd0f:	62 f1 7c 48 10 47 10 	vmovups zmm0,ZMMWORD PTR [rdi+0x400]
  67fd16:	62 f1 7c 48 11 44 24 	vmovups ZMMWORD PTR [rsp+0x1c0],zmm0
  67fd1d:	07 
  67fd1e:	62 f1 7c 48 10 47 11 	vmovups zmm0,ZMMWORD PTR [rdi+0x440]
  67fd25:	62 f1 7c 48 11 44 24 	vmovups ZMMWORD PTR [rsp+0x200],zmm0
  67fd2c:	08 
  67fd2d:	62 e1 fe 48 6f 7f 14 	vmovdqu64 zmm23,ZMMWORD PTR [rdi+0x500]
  67fd34:	62 f1 7c 48 10 47 15 	vmovups zmm0,ZMMWORD PTR [rdi+0x540]
  67fd3b:	62 f1 7c 48 11 44 24 	vmovups ZMMWORD PTR [rsp+0x140],zmm0
  67fd42:	05 
  67fd43:	62 e1 fe 48 6f 6f 18 	vmovdqu64 zmm21,ZMMWORD PTR [rdi+0x600]
  67fd4a:	62 e1 fe 48 6f 67 19 	vmovdqu64 zmm20,ZMMWORD PTR [rdi+0x640]
  67fd51:	62 f1 7c 48 10 47 1c 	vmovups zmm0,ZMMWORD PTR [rdi+0x700]
  67fd58:	62 f1 7c 48 11 44 24 	vmovups ZMMWORD PTR [rsp+0x100],zmm0
  67fd5f:	04 
  67fd60:	62 e1 fe 48 6f 57 1d 	vmovdqu64 zmm18,ZMMWORD PTR [rdi+0x740]
  67fd67:	62 e1 fe 48 6f 77 20 	vmovdqu64 zmm22,ZMMWORD PTR [rdi+0x800]
  67fd6e:	62 f1 fe 48 6f 77 21 	vmovdqu64 zmm6,ZMMWORD PTR [rdi+0x840]
  67fd75:	62 e1 fe 48 6f 5f 24 	vmovdqu64 zmm19,ZMMWORD PTR [rdi+0x900]
  67fd7c:	62 e1 fe 48 6f 47 25 	vmovdqu64 zmm16,ZMMWORD PTR [rdi+0x940]
  67fd83:	62 71 fe 48 6f 6f 28 	vmovdqu64 zmm13,ZMMWORD PTR [rdi+0xa00]
  67fd8a:	62 71 fe 48 6f 77 29 	vmovdqu64 zmm14,ZMMWORD PTR [rdi+0xa40]
  67fd91:	62 71 fe 48 6f 4f 2c 	vmovdqu64 zmm9,ZMMWORD PTR [rdi+0xb00]
  67fd98:	62 71 fe 48 6f 57 2d 	vmovdqu64 zmm10,ZMMWORD PTR [rdi+0xb40]
  67fd9f:	62 e1 fe 48 6f 4f 30 	vmovdqu64 zmm17,ZMMWORD PTR [rdi+0xc00]
  67fda6:	62 71 fe 48 6f 47 31 	vmovdqu64 zmm8,ZMMWORD PTR [rdi+0xc40]
  67fdad:	62 f1 fe 48 6f 7f 34 	vmovdqu64 zmm7,ZMMWORD PTR [rdi+0xd00]
  67fdb4:	62 71 fe 48 6f 7f 35 	vmovdqu64 zmm15,ZMMWORD PTR [rdi+0xd40]
  67fdbb:	62 f1 fe 48 6f 6f 38 	vmovdqu64 zmm5,ZMMWORD PTR [rdi+0xe00]
  67fdc2:	62 f1 fe 48 6f 67 39 	vmovdqu64 zmm4,ZMMWORD PTR [rdi+0xe40]
  67fdc9:	62 f1 fe 48 6f 5f 3c 	vmovdqu64 zmm3,ZMMWORD PTR [rdi+0xf00]
  67fdd0:	62 f1 7c 48 10 47 3d 	vmovups zmm0,ZMMWORD PTR [rdi+0xf40]
  67fdd7:	49 c1 e1 07          	shl    r9,0x7
  67fddb:	45 31 d2             	xor    r10d,r10d
  67fdde:	66 90                	xchg   ax,ax
  67fde0:	62 71 fe 48 7f 7c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x280],zmm15
  67fde7:	0a 
  67fde8:	62 f1 fe 48 7f 7c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x2c0],zmm7
  67fdef:	0b 
  67fdf0:	62 71 fe 48 7f 54 24 	vmovdqu64 ZMMWORD PTR [rsp+0x300],zmm10
  67fdf7:	0c 
  67fdf8:	62 e1 fe 48 7f 5c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x340],zmm19
  67fdff:	0d 
  67fe00:	62 61 fe 48 7f 7c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x380],zmm31
  67fe07:	0e 
  67fe08:	62 e1 fe 48 7f 7c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x3c0],zmm23
  67fe0f:	0f 
  67fe10:	62 e1 fe 48 7f 54 24 	vmovdqu64 ZMMWORD PTR [rsp+0x400],zmm18
  67fe17:	10 
  67fe18:	62 71 fe 48 7f 44 24 	vmovdqu64 ZMMWORD PTR [rsp+0x440],zmm8
  67fe1f:	11 
  67fe20:	62 71 fe 48 7f 6c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x480],zmm13
  67fe27:	12 
  67fe28:	62 f1 fe 48 7f 6c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x4c0],zmm5
  67fe2f:	13 
  67fe30:	62 f1 fe 48 7f 64 24 	vmovdqu64 ZMMWORD PTR [rsp+0x500],zmm4
  67fe37:	14 
  67fe38:	62 f1 fe 48 7f 5c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x540],zmm3
  67fe3f:	15 
  67fe40:	62 f1 7c 48 11 44 24 	vmovups ZMMWORD PTR [rsp+0x80],zmm0
  67fe47:	02 
  67fe48:	c4 a1 7a 6f 04 11    	vmovdqu xmm0,XMMWORD PTR [rcx+r10*1]
  67fe4e:	c4 a1 7a 6f 4c 11 10 	vmovdqu xmm1,XMMWORD PTR [rcx+r10*1+0x10]
  67fe55:	c4 a1 7a 6f 54 11 20 	vmovdqu xmm2,XMMWORD PTR [rcx+r10*1+0x20]
  67fe5c:	c4 a1 7a 6f 5c 11 30 	vmovdqu xmm3,XMMWORD PTR [rcx+r10*1+0x30]
  67fe63:	c4 a1 7a 6f 64 11 40 	vmovdqu xmm4,XMMWORD PTR [rcx+r10*1+0x40]
  67fe6a:	c4 a1 7a 6f 6c 11 50 	vmovdqu xmm5,XMMWORD PTR [rcx+r10*1+0x50]
  67fe71:	62 01 fd 48 6f fa    	vmovdqa64 zmm31,zmm26
  67fe77:	62 81 fd 48 6f fe    	vmovdqa64 zmm23,zmm30
  67fe7d:	62 21 fd 48 6f f5    	vmovdqa64 zmm30,zmm21
  67fe83:	62 61 fd 48 6f d6    	vmovdqa64 zmm26,zmm6
  67fe89:	62 c1 fd 48 6f e9    	vmovdqa64 zmm21,zmm9
  67fe8f:	c4 a1 7a 6f 74 11 60 	vmovdqu xmm6,XMMWORD PTR [rcx+r10*1+0x60]
  67fe96:	c4 a1 7a 6f 7c 11 70 	vmovdqu xmm7,XMMWORD PTR [rcx+r10*1+0x70]
  67fe9d:	c5 79 68 c1          	vpunpckhbw xmm8,xmm0,xmm1
  67fea1:	c5 69 68 cb          	vpunpckhbw xmm9,xmm2,xmm3
  67fea5:	c5 59 68 d5          	vpunpckhbw xmm10,xmm4,xmm5
  67fea9:	c5 f9 60 c1          	vpunpcklbw xmm0,xmm0,xmm1
  67fead:	c5 c9 68 cf          	vpunpckhbw xmm1,xmm6,xmm7
  67feb1:	c5 e9 60 d3          	vpunpcklbw xmm2,xmm2,xmm3
  67feb5:	c5 d9 60 dd          	vpunpcklbw xmm3,xmm4,xmm5
  67feb9:	c5 c9 60 e7          	vpunpcklbw xmm4,xmm6,xmm7
  67febd:	c5 f9 61 ea          	vpunpcklwd xmm5,xmm0,xmm2
  67fec1:	c5 f9 69 c2          	vpunpckhwd xmm0,xmm0,xmm2
  67fec5:	c5 e1 61 d4          	vpunpcklwd xmm2,xmm3,xmm4
  67fec9:	c5 e1 69 dc          	vpunpckhwd xmm3,xmm3,xmm4
  67fecd:	c4 c1 39 61 e1       	vpunpcklwd xmm4,xmm8,xmm9
  67fed2:	c4 c1 39 69 f1       	vpunpckhwd xmm6,xmm8,xmm9
  67fed7:	c5 a9 61 f9          	vpunpcklwd xmm7,xmm10,xmm1
  67fedb:	c5 a9 69 c9          	vpunpckhwd xmm1,xmm10,xmm1
  67fedf:	c5 7d 6f 2d b9 d4 98 	vmovdqa ymm13,YMMWORD PTR [rip+0xffffffffff98d4b9]        # d3a0 <anon.36d21461a6996b7f2bd9a76e3225918f.28.llvm.7983859039285366149+0x100>
  67fee6:	ff 
  67fee7:	62 f2 15 28 7e ea    	vpermt2d ymm5,ymm13,ymm2
  67feed:	c5 f9 62 d3          	vpunpckldq xmm2,xmm0,xmm3
  67fef1:	62 f3 55 48 38 d2 02 	vinserti32x4 zmm2,zmm5,xmm2,0x2
  67fef8:	c5 f9 6a c3          	vpunpckhdq xmm0,xmm0,xmm3
  67fefc:	62 73 6d 48 38 e0 03 	vinserti32x4 zmm12,zmm2,xmm0,0x3
  67ff03:	62 f2 15 28 7e e7    	vpermt2d ymm4,ymm13,ymm7
  67ff09:	c5 c9 62 d1          	vpunpckldq xmm2,xmm6,xmm1
  67ff0d:	62 f3 5d 48 38 d2 02 	vinserti32x4 zmm2,zmm4,xmm2,0x2
  67ff14:	c5 c9 6a c9          	vpunpckhdq xmm1,xmm6,xmm1
  67ff18:	62 f3 6d 48 38 c1 03 	vinserti32x4 zmm0,zmm2,xmm1,0x3
  67ff1f:	62 f1 fe 48 7f 44 24 	vmovdqu64 ZMMWORD PTR [rsp+0xc0],zmm0
  67ff26:	03 
  67ff27:	c4 a1 7a 6f 14 16    	vmovdqu xmm2,XMMWORD PTR [rsi+r10*1]
  67ff2d:	c4 a1 7a 6f 5c 16 10 	vmovdqu xmm3,XMMWORD PTR [rsi+r10*1+0x10]
  67ff34:	c4 a1 7a 6f 64 16 20 	vmovdqu xmm4,XMMWORD PTR [rsi+r10*1+0x20]
  67ff3b:	c4 a1 7a 6f 6c 16 30 	vmovdqu xmm5,XMMWORD PTR [rsi+r10*1+0x30]
  67ff42:	c4 a1 7a 6f 74 16 40 	vmovdqu xmm6,XMMWORD PTR [rsi+r10*1+0x40]
  67ff49:	c4 a1 7a 6f 7c 16 50 	vmovdqu xmm7,XMMWORD PTR [rsi+r10*1+0x50]
  67ff50:	c4 21 7a 6f 44 16 60 	vmovdqu xmm8,XMMWORD PTR [rsi+r10*1+0x60]
  67ff57:	c4 21 7a 6f 4c 16 70 	vmovdqu xmm9,XMMWORD PTR [rsi+r10*1+0x70]
  67ff5e:	c5 69 68 d3          	vpunpckhbw xmm10,xmm2,xmm3
  67ff62:	c5 e9 60 d3          	vpunpcklbw xmm2,xmm2,xmm3
  67ff66:	c5 d9 68 dd          	vpunpckhbw xmm3,xmm4,xmm5
  67ff6a:	c5 d9 60 e5          	vpunpcklbw xmm4,xmm4,xmm5
  67ff6e:	c5 c9 68 ef          	vpunpckhbw xmm5,xmm6,xmm7
  67ff72:	c5 c9 60 f7          	vpunpcklbw xmm6,xmm6,xmm7
  67ff76:	c4 c1 39 68 f9       	vpunpckhbw xmm7,xmm8,xmm9
  67ff7b:	c4 41 39 60 c1       	vpunpcklbw xmm8,xmm8,xmm9
  67ff80:	c5 69 61 cc          	vpunpcklwd xmm9,xmm2,xmm4
  67ff84:	c5 e9 69 d4          	vpunpckhwd xmm2,xmm2,xmm4
  67ff88:	c4 c1 49 61 e0       	vpunpcklwd xmm4,xmm6,xmm8
  67ff8d:	c4 c1 49 69 f0       	vpunpckhwd xmm6,xmm6,xmm8
  67ff92:	c5 29 61 c3          	vpunpcklwd xmm8,xmm10,xmm3
  67ff96:	c5 a9 69 db          	vpunpckhwd xmm3,xmm10,xmm3
  67ff9a:	c5 51 61 d7          	vpunpcklwd xmm10,xmm5,xmm7
  67ff9e:	c5 d1 69 ef          	vpunpckhwd xmm5,xmm5,xmm7
  67ffa2:	62 61 fe 48 6f 5c 24 	vmovdqu64 zmm27,ZMMWORD PTR [rsp+0x180]
  67ffa9:	06 
  67ffaa:	62 71 fe 48 6f 5c 24 	vmovdqu64 zmm11,ZMMWORD PTR [rsp+0x140]
  67ffb1:	05 
  67ffb2:	62 72 15 28 7e cc    	vpermt2d ymm9,ymm13,ymm4
  67ffb8:	c5 e9 62 e6          	vpunpckldq xmm4,xmm2,xmm6
  67ffbc:	62 f3 35 48 38 e4 02 	vinserti32x4 zmm4,zmm9,xmm4,0x2
  67ffc3:	62 61 fe 48 6f 6c 24 	vmovdqu64 zmm29,ZMMWORD PTR [rsp+0x240]
  67ffca:	09 
  67ffcb:	62 61 fe 48 6f 44 24 	vmovdqu64 zmm24,ZMMWORD PTR [rsp+0x200]
  67ffd2:	08 
  67ffd3:	62 71 fe 48 6f 7c 24 	vmovdqu64 zmm15,ZMMWORD PTR [rsp+0x100]
  67ffda:	04 
  67ffdb:	c5 e9 6a d6          	vpunpckhdq xmm2,xmm2,xmm6
  67ffdf:	62 31 fd 48 6f cd    	vmovdqa64 zmm9,zmm21
  67ffe5:	62 91 fd 48 6f f2    	vmovdqa64 zmm6,zmm26
  67ffeb:	62 81 fd 48 6f ee    	vmovdqa64 zmm21,zmm30
  67fff1:	62 21 fd 48 6f f7    	vmovdqa64 zmm30,zmm23
  67fff7:	62 01 fd 48 6f d7    	vmovdqa64 zmm26,zmm31
  67fffd:	62 61 fe 48 6f 7c 24 	vmovdqu64 zmm31,ZMMWORD PTR [rsp+0x380]
  680004:	0e 
  680005:	62 f3 5d 48 38 d2 03 	vinserti32x4 zmm2,zmm4,xmm2,0x3
  68000c:	62 52 15 28 7e c2    	vpermt2d ymm8,ymm13,ymm10
  680012:	62 71 fe 48 6f 54 24 	vmovdqu64 zmm10,ZMMWORD PTR [rsp+0x300]
  680019:	0c 
  68001a:	62 e1 fe 48 6f 5c 24 	vmovdqu64 zmm19,ZMMWORD PTR [rsp+0x340]
  680021:	0d 
  680022:	62 61 fe 48 6f 4c 24 	vmovdqu64 zmm25,ZMMWORD PTR [rsp+0x1c0]
  680029:	07 
  68002a:	c5 e1 62 e5          	vpunpckldq xmm4,xmm3,xmm5
  68002e:	62 f3 3d 48 38 e4 02 	vinserti32x4 zmm4,zmm8,xmm4,0x2
  680035:	62 71 fe 48 6f 44 24 	vmovdqu64 zmm8,ZMMWORD PTR [rsp+0x440]
  68003c:	11 
  68003d:	62 71 fe 48 6f 6c 24 	vmovdqu64 zmm13,ZMMWORD PTR [rsp+0x480]
  680044:	12 
  680045:	62 e1 fe 48 6f 7c 24 	vmovdqu64 zmm23,ZMMWORD PTR [rsp+0x3c0]
  68004c:	0f 
  68004d:	62 e1 fe 48 6f 54 24 	vmovdqu64 zmm18,ZMMWORD PTR [rsp+0x400]
  680054:	10 
  680055:	62 f2 7d 48 5a 3d f1 	vbroadcasti32x4 zmm7,XMMWORD PTR [rip+0xffffffffff98c7f1]        # c850 <anon.0fac96d756d24b1a23b798945b2a13ad.103.llvm.5929972809416623235+0x118>
  68005c:	c7 98 ff 
  68005f:	62 f2 1d 48 00 c7    	vpshufb zmm0,zmm12,zmm7
  680065:	62 72 fd 48 59 25 e9 	vpbroadcastq zmm12,QWORD PTR [rip+0xffffffffff98cce9]        # cd58 <anon.36d21461a6996b7f2bd9a76e3225918f.29.llvm.7983859039285366149+0x138>
  68006c:	cc 98 ff 
  68006f:	62 f3 9d 48 ce c0 00 	vgf2p8affineqb zmm0,zmm12,zmm0,0x0
  680076:	62 f1 fe 48 6f 4c 24 	vmovdqu64 zmm1,ZMMWORD PTR [rsp+0xc0]
  68007d:	03 
  68007e:	62 f2 75 48 00 cf    	vpshufb zmm1,zmm1,zmm7
  680084:	62 f3 9d 48 ce c9 00 	vgf2p8affineqb zmm1,zmm12,zmm1,0x0
  68008b:	62 f2 6d 48 00 d7    	vpshufb zmm2,zmm2,zmm7
  680091:	62 72 fd 48 59 25 7d 	vpbroadcastq zmm12,QWORD PTR [rip+0xffffffffff98cc7d]        # cd18 <anon.36d21461a6996b7f2bd9a76e3225918f.29.llvm.7983859039285366149+0xf8>
  680098:	cc 98 ff 
  68009b:	62 f3 9d 48 ce d2 00 	vgf2p8affineqb zmm2,zmm12,zmm2,0x0
  6800a2:	c5 e1 6a dd          	vpunpckhdq xmm3,xmm3,xmm5
  6800a6:	62 f3 5d 48 38 db 03 	vinserti32x4 zmm3,zmm4,xmm3,0x3
  6800ad:	62 f1 fe 48 7f 5c 24 	vmovdqu64 ZMMWORD PTR [rsp+0xc0],zmm3
  6800b4:	03 
  6800b5:	62 f2 fd 48 59 e2    	vpbroadcastq zmm4,xmm2
  6800bb:	62 f3 fd 48 ce ec 00 	vgf2p8affineqb zmm5,zmm0,zmm4,0x0
  6800c2:	62 f1 fe 48 6f 1c 24 	vmovdqu64 zmm3,ZMMWORD PTR [rsp]
  6800c9:	62 f1 65 48 ef dd    	vpxord zmm3,zmm3,zmm5
  6800cf:	62 f1 fe 48 7f 1c 24 	vmovdqu64 ZMMWORD PTR [rsp],zmm3
  6800d6:	62 f3 f5 48 ce e4 00 	vgf2p8affineqb zmm4,zmm1,zmm4,0x0
  6800dd:	62 f1 fe 48 6f 6c 24 	vmovdqu64 zmm5,ZMMWORD PTR [rsp+0x40]
  6800e4:	01 
  6800e5:	62 f1 55 48 ef ec    	vpxord zmm5,zmm5,zmm4
  6800eb:	62 f1 fe 48 7f 6c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x40],zmm5
  6800f2:	01 
  6800f3:	62 f2 fd 48 59 1d 73 	vpbroadcastq zmm3,QWORD PTR [rip+0xffffffffff98cc73]        # cd70 <anon.36d21461a6996b7f2bd9a76e3225918f.29.llvm.7983859039285366149+0x150>
  6800fa:	cc 98 ff 
  6800fd:	62 f2 e5 48 36 e2    	vpermq zmm4,zmm3,zmm2
  680103:	62 f3 fd 48 ce ec 00 	vgf2p8affineqb zmm5,zmm0,zmm4,0x0
  68010a:	62 61 05 40 ef fd    	vpxord zmm31,zmm31,zmm5
  680110:	62 f3 f5 48 ce e4 00 	vgf2p8affineqb zmm4,zmm1,zmm4,0x0
  680117:	62 61 0d 40 ef f4    	vpxord zmm30,zmm30,zmm4
  68011d:	62 f2 fd 48 59 1d b1 	vpbroadcastq zmm3,QWORD PTR [rip+0xffffffffff98ccb1]        # cdd8 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0x58>
  680124:	cc 98 ff 
  680127:	62 f2 e5 48 36 e2    	vpermq zmm4,zmm3,zmm2
  68012d:	62 f3 fd 48 ce ec 00 	vgf2p8affineqb zmm5,zmm0,zmm4,0x0
  680134:	62 61 15 40 ef ed    	vpxord zmm29,zmm29,zmm5
  68013a:	62 61 fe 48 7f 6c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x240],zmm29
  680141:	09 
  680142:	62 f3 f5 48 ce e4 00 	vgf2p8affineqb zmm4,zmm1,zmm4,0x0
  680149:	62 61 1d 40 ef e4    	vpxord zmm28,zmm28,zmm4
  68014f:	62 62 fd 48 59 2d cf 	vpbroadcastq zmm29,QWORD PTR [rip+0xffffffffff98cccf]        # ce28 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0xa8>
  680156:	cc 98 ff 
  680159:	62 f2 95 40 36 e2    	vpermq zmm4,zmm29,zmm2
  68015f:	62 f3 fd 48 ce ec 00 	vgf2p8affineqb zmm5,zmm0,zmm4,0x0
  680166:	62 61 25 40 ef dd    	vpxord zmm27,zmm27,zmm5
  68016c:	62 61 fe 48 7f 5c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x180],zmm27
  680173:	06 
  680174:	62 f3 f5 48 ce e4 00 	vgf2p8affineqb zmm4,zmm1,zmm4,0x0
  68017b:	62 61 2d 40 ef d4    	vpxord zmm26,zmm26,zmm4
  680181:	62 62 fd 48 59 1d bd 	vpbroadcastq zmm27,QWORD PTR [rip+0xffffffffff98ccbd]        # ce48 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0xc8>
  680188:	cc 98 ff 
  68018b:	62 f2 a5 40 36 e2    	vpermq zmm4,zmm27,zmm2
  680191:	62 f3 fd 48 ce ec 00 	vgf2p8affineqb zmm5,zmm0,zmm4,0x0
  680198:	62 61 35 40 ef cd    	vpxord zmm25,zmm25,zmm5
  68019e:	62 61 fe 48 7f 4c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x1c0],zmm25
  6801a5:	07 
  6801a6:	62 f3 f5 48 ce e4 00 	vgf2p8affineqb zmm4,zmm1,zmm4,0x0
  6801ad:	62 61 3d 40 ef c4    	vpxord zmm24,zmm24,zmm4
  6801b3:	62 61 fe 48 7f 44 24 	vmovdqu64 ZMMWORD PTR [rsp+0x200],zmm24
  6801ba:	08 
  6801bb:	62 62 fd 48 59 0d 63 	vpbroadcastq zmm25,QWORD PTR [rip+0xffffffffff98cb63]        # cd28 <anon.36d21461a6996b7f2bd9a76e3225918f.29.llvm.7983859039285366149+0x108>
  6801c2:	cb 98 ff 
  6801c5:	62 f2 b5 40 36 e2    	vpermq zmm4,zmm25,zmm2
  6801cb:	62 f3 fd 48 ce ec 00 	vgf2p8affineqb zmm5,zmm0,zmm4,0x0
  6801d2:	62 e1 45 40 ef fd    	vpxord zmm23,zmm23,zmm5
  6801d8:	62 f3 f5 48 ce e4 00 	vgf2p8affineqb zmm4,zmm1,zmm4,0x0
  6801df:	62 71 25 48 ef dc    	vpxord zmm11,zmm11,zmm4
  6801e5:	62 71 fe 48 7f 5c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x140],zmm11
  6801ec:	05 
  6801ed:	62 62 fd 48 59 05 49 	vpbroadcastq zmm24,QWORD PTR [rip+0xffffffffff98cb49]        # cd40 <anon.36d21461a6996b7f2bd9a76e3225918f.29.llvm.7983859039285366149+0x120>
  6801f4:	cb 98 ff 
  6801f7:	62 f2 bd 40 36 e2    	vpermq zmm4,zmm24,zmm2
  6801fd:	62 f3 fd 48 ce ec 00 	vgf2p8affineqb zmm5,zmm0,zmm4,0x0
  680204:	62 e1 55 40 ef ed    	vpxord zmm21,zmm21,zmm5
  68020a:	62 f1 fe 48 6f 6c 24 	vmovdqu64 zmm5,ZMMWORD PTR [rsp+0x4c0]
  680211:	13 
  680212:	62 f1 fe 48 6f 5c 24 	vmovdqu64 zmm3,ZMMWORD PTR [rsp+0xc0]
  680219:	03 
  68021a:	62 f2 65 48 00 df    	vpshufb zmm3,zmm3,zmm7
  680220:	62 f3 9d 48 ce db 00 	vgf2p8affineqb zmm3,zmm12,zmm3,0x0
  680227:	62 f3 f5 48 ce e4 00 	vgf2p8affineqb zmm4,zmm1,zmm4,0x0
  68022e:	62 e1 5d 40 ef e4    	vpxord zmm20,zmm20,zmm4
  680234:	62 72 fd 48 59 25 02 	vpbroadcastq zmm12,QWORD PTR [rip+0xffffffffff98cc02]        # ce40 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0xc0>
  68023b:	cc 98 ff 
  68023e:	62 f2 9d 48 36 d2    	vpermq zmm2,zmm12,zmm2
  680244:	62 f3 fd 48 ce e2 00 	vgf2p8affineqb zmm4,zmm0,zmm2,0x0
  68024b:	62 71 05 48 ef fc    	vpxord zmm15,zmm15,zmm4
  680251:	62 71 fe 48 7f 7c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x100],zmm15
  680258:	04 
  680259:	62 f3 f5 48 ce d2 00 	vgf2p8affineqb zmm2,zmm1,zmm2,0x0
  680260:	62 e1 6d 40 ef d2    	vpxord zmm18,zmm18,zmm2
  680266:	62 f2 fd 48 59 d3    	vpbroadcastq zmm2,xmm3
  68026c:	62 f3 fd 48 ce e2 00 	vgf2p8affineqb zmm4,zmm0,zmm2,0x0
  680273:	62 e1 4d 40 ef f4    	vpxord zmm22,zmm22,zmm4
  680279:	62 f3 f5 48 ce d2 00 	vgf2p8affineqb zmm2,zmm1,zmm2,0x0
  680280:	62 f1 4d 48 ef f2    	vpxord zmm6,zmm6,zmm2
  680286:	62 f2 fd 48 59 15 e0 	vpbroadcastq zmm2,QWORD PTR [rip+0xffffffffff98cae0]        # cd70 <anon.36d21461a6996b7f2bd9a76e3225918f.29.llvm.7983859039285366149+0x150>
  68028d:	ca 98 ff 
  680290:	62 f2 ed 48 36 d3    	vpermq zmm2,zmm2,zmm3
  680296:	62 f3 fd 48 ce e2 00 	vgf2p8affineqb zmm4,zmm0,zmm2,0x0
  68029d:	62 e1 65 40 ef dc    	vpxord zmm19,zmm19,zmm4
  6802a3:	62 f3 f5 48 ce d2 00 	vgf2p8affineqb zmm2,zmm1,zmm2,0x0
  6802aa:	62 e1 7d 40 ef c2    	vpxord zmm16,zmm16,zmm2
  6802b0:	62 f2 fd 48 59 15 1e 	vpbroadcastq zmm2,QWORD PTR [rip+0xffffffffff98cb1e]        # cdd8 <anon.36d21461a6996b7f2bd9a76e3225918f.11.llvm.7983859039285366149+0x58>
  6802b7:	cb 98 ff 
  6802ba:	62 f2 ed 48 36 d3    	vpermq zmm2,zmm2,zmm3
  6802c0:	62 f3 fd 48 ce e2 00 	vgf2p8affineqb zmm4,zmm0,zmm2,0x0
  6802c7:	62 71 15 48 ef ec    	vpxord zmm13,zmm13,zmm4
  6802cd:	62 f3 f5 48 ce d2 00 	vgf2p8affineqb zmm2,zmm1,zmm2,0x0
  6802d4:	62 71 0d 48 ef f2    	vpxord zmm14,zmm14,zmm2
  6802da:	62 f2 95 40 36 d3    	vpermq zmm2,zmm29,zmm3
  6802e0:	62 f3 fd 48 ce e2 00 	vgf2p8affineqb zmm4,zmm0,zmm2,0x0
  6802e7:	62 71 35 48 ef cc    	vpxord zmm9,zmm9,zmm4
  6802ed:	62 f3 f5 48 ce d2 00 	vgf2p8affineqb zmm2,zmm1,zmm2,0x0
  6802f4:	62 71 2d 48 ef d2    	vpxord zmm10,zmm10,zmm2
  6802fa:	62 f2 a5 40 36 d3    	vpermq zmm2,zmm27,zmm3
  680300:	62 f3 fd 48 ce e2 00 	vgf2p8affineqb zmm4,zmm0,zmm2,0x0
  680307:	62 e1 75 40 ef cc    	vpxord zmm17,zmm17,zmm4
  68030d:	62 f3 f5 48 ce d2 00 	vgf2p8affineqb zmm2,zmm1,zmm2,0x0
  680314:	62 71 3d 48 ef c2    	vpxord zmm8,zmm8,zmm2
  68031a:	62 f2 b5 40 36 d3    	vpermq zmm2,zmm25,zmm3
  680320:	62 f3 fd 48 ce e2 00 	vgf2p8affineqb zmm4,zmm0,zmm2,0x0
  680327:	62 f1 fe 48 6f 7c 24 	vmovdqu64 zmm7,ZMMWORD PTR [rsp+0x2c0]
  68032e:	0b 
  68032f:	62 f1 45 48 ef fc    	vpxord zmm7,zmm7,zmm4
  680335:	62 f3 f5 48 ce d2 00 	vgf2p8affineqb zmm2,zmm1,zmm2,0x0
  68033c:	62 71 fe 48 6f 7c 24 	vmovdqu64 zmm15,ZMMWORD PTR [rsp+0x280]
  680343:	0a 
  680344:	62 71 05 48 ef fa    	vpxord zmm15,zmm15,zmm2
  68034a:	62 f2 bd 40 36 d3    	vpermq zmm2,zmm24,zmm3
  680350:	62 f3 fd 48 ce e2 00 	vgf2p8affineqb zmm4,zmm0,zmm2,0x0
  680357:	62 f1 55 48 ef ec    	vpxord zmm5,zmm5,zmm4
  68035d:	62 f1 fe 48 6f 64 24 	vmovdqu64 zmm4,ZMMWORD PTR [rsp+0x500]
  680364:	14 
  680365:	62 f3 f5 48 ce d2 00 	vgf2p8affineqb zmm2,zmm1,zmm2,0x0
  68036c:	62 f1 5d 48 ef e2    	vpxord zmm4,zmm4,zmm2
  680372:	62 f2 9d 48 36 d3    	vpermq zmm2,zmm12,zmm3
  680378:	62 f1 fe 48 6f 5c 24 	vmovdqu64 zmm3,ZMMWORD PTR [rsp+0x540]
  68037f:	15 
  680380:	62 f3 fd 48 ce c2 00 	vgf2p8affineqb zmm0,zmm0,zmm2,0x0
  680387:	62 f1 65 48 ef d8    	vpxord zmm3,zmm3,zmm0
  68038d:	62 f3 f5 48 ce c2 00 	vgf2p8affineqb zmm0,zmm1,zmm2,0x0
  680394:	62 f1 fe 48 6f 4c 24 	vmovdqu64 zmm1,ZMMWORD PTR [rsp+0x80]
  68039b:	02 
  68039c:	62 f1 75 48 ef c8    	vpxord zmm1,zmm1,zmm0
  6803a2:	62 f1 fe 48 7f 4c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x80],zmm1
  6803a9:	02 
  6803aa:	62 f1 7c 48 10 44 24 	vmovups zmm0,ZMMWORD PTR [rsp+0x80]
  6803b1:	02 
  6803b2:	49 83 ea 80          	sub    r10,0xffffffffffffff80
  6803b6:	4d 39 d1             	cmp    r9,r10
  6803b9:	0f 85 21 fa ff ff    	jne    67fde0 <_RNvNtNtNtCs5Palesvl9wF_6mikros10univariate5tiled8affine_h16consume_h_affine+0x140>
  6803bf:	62 f1 7c 48 10 0c 24 	vmovups zmm1,ZMMWORD PTR [rsp]
  6803c6:	62 f1 7c 48 11 0f    	vmovups ZMMWORD PTR [rdi],zmm1
  6803cc:	62 f1 7c 48 10 4c 24 	vmovups zmm1,ZMMWORD PTR [rsp+0x40]
  6803d3:	01 
  6803d4:	62 f1 7c 48 11 4f 01 	vmovups ZMMWORD PTR [rdi+0x40],zmm1
  6803db:	62 61 fe 48 7f 7f 04 	vmovdqu64 ZMMWORD PTR [rdi+0x100],zmm31
  6803e2:	62 61 fe 48 7f 77 05 	vmovdqu64 ZMMWORD PTR [rdi+0x140],zmm30
  6803e9:	62 f1 7c 48 10 4c 24 	vmovups zmm1,ZMMWORD PTR [rsp+0x240]
  6803f0:	09 
  6803f1:	62 f1 7c 48 11 4f 08 	vmovups ZMMWORD PTR [rdi+0x200],zmm1
  6803f8:	62 61 fe 48 7f 67 09 	vmovdqu64 ZMMWORD PTR [rdi+0x240],zmm28
  6803ff:	62 f1 7c 48 10 4c 24 	vmovups zmm1,ZMMWORD PTR [rsp+0x180]
  680406:	06 
  680407:	62 f1 7c 48 11 4f 0c 	vmovups ZMMWORD PTR [rdi+0x300],zmm1
  68040e:	62 61 fe 48 7f 57 0d 	vmovdqu64 ZMMWORD PTR [rdi+0x340],zmm26
  680415:	62 f1 7c 48 10 4c 24 	vmovups zmm1,ZMMWORD PTR [rsp+0x1c0]
  68041c:	07 
  68041d:	62 f1 7c 48 11 4f 10 	vmovups ZMMWORD PTR [rdi+0x400],zmm1
  680424:	62 f1 7c 48 10 4c 24 	vmovups zmm1,ZMMWORD PTR [rsp+0x200]
  68042b:	08 
  68042c:	62 f1 7c 48 11 4f 11 	vmovups ZMMWORD PTR [rdi+0x440],zmm1
  680433:	62 e1 fe 48 7f 7f 14 	vmovdqu64 ZMMWORD PTR [rdi+0x500],zmm23
  68043a:	62 f1 7c 48 10 4c 24 	vmovups zmm1,ZMMWORD PTR [rsp+0x140]
  680441:	05 
  680442:	62 f1 7c 48 11 4f 15 	vmovups ZMMWORD PTR [rdi+0x540],zmm1
  680449:	62 e1 fe 48 7f 6f 18 	vmovdqu64 ZMMWORD PTR [rdi+0x600],zmm21
  680450:	62 e1 fe 48 7f 67 19 	vmovdqu64 ZMMWORD PTR [rdi+0x640],zmm20
  680457:	62 f1 fe 48 6f 4c 24 	vmovdqu64 zmm1,ZMMWORD PTR [rsp+0x100]
  68045e:	04 
  68045f:	62 f1 fe 48 7f 4f 1c 	vmovdqu64 ZMMWORD PTR [rdi+0x700],zmm1
  680466:	62 e1 fe 48 7f 57 1d 	vmovdqu64 ZMMWORD PTR [rdi+0x740],zmm18
  68046d:	62 e1 fe 48 7f 77 20 	vmovdqu64 ZMMWORD PTR [rdi+0x800],zmm22
  680474:	62 f1 fe 48 7f 77 21 	vmovdqu64 ZMMWORD PTR [rdi+0x840],zmm6
  68047b:	62 e1 fe 48 7f 5f 24 	vmovdqu64 ZMMWORD PTR [rdi+0x900],zmm19
  680482:	62 e1 fe 48 7f 47 25 	vmovdqu64 ZMMWORD PTR [rdi+0x940],zmm16
  680489:	62 71 fe 48 7f 6f 28 	vmovdqu64 ZMMWORD PTR [rdi+0xa00],zmm13
  680490:	62 71 fe 48 7f 77 29 	vmovdqu64 ZMMWORD PTR [rdi+0xa40],zmm14
  680497:	62 71 fe 48 7f 4f 2c 	vmovdqu64 ZMMWORD PTR [rdi+0xb00],zmm9
  68049e:	62 71 fe 48 7f 57 2d 	vmovdqu64 ZMMWORD PTR [rdi+0xb40],zmm10
  6804a5:	62 e1 fe 48 7f 4f 30 	vmovdqu64 ZMMWORD PTR [rdi+0xc00],zmm17
  6804ac:	62 71 fe 48 7f 47 31 	vmovdqu64 ZMMWORD PTR [rdi+0xc40],zmm8
  6804b3:	62 f1 fe 48 7f 7f 34 	vmovdqu64 ZMMWORD PTR [rdi+0xd00],zmm7
  6804ba:	62 71 fe 48 7f 7f 35 	vmovdqu64 ZMMWORD PTR [rdi+0xd40],zmm15
  6804c1:	62 f1 fe 48 7f 6f 38 	vmovdqu64 ZMMWORD PTR [rdi+0xe00],zmm5
  6804c8:	62 f1 fe 48 7f 67 39 	vmovdqu64 ZMMWORD PTR [rdi+0xe40],zmm4
  6804cf:	62 f1 fe 48 7f 5f 3c 	vmovdqu64 ZMMWORD PTR [rdi+0xf00],zmm3
  6804d6:	62 f1 7c 48 11 47 3d 	vmovups ZMMWORD PTR [rdi+0xf40],zmm0
  6804dd:	4d 89 c1             	mov    r9,r8
  6804e0:	49 29 c1             	sub    r9,rax
  6804e3:	0f 82 bb 05 00 00    	jb     680aa4 <_RNvNtNtNtCs5Palesvl9wF_6mikros10univariate5tiled8affine_h16consume_h_affine+0xe04>
  6804e9:	89 d0                	mov    eax,edx
  6804eb:	83 e0 07             	and    eax,0x7
  6804ee:	49 39 c1             	cmp    r9,rax
  6804f1:	49 0f 42 c1          	cmovb  rax,r9
  6804f5:	48 85 c0             	test   rax,rax
  6804f8:	0f 84 9b 05 00 00    	je     680a99 <_RNvNtNtNtCs5Palesvl9wF_6mikros10univariate5tiled8affine_h16consume_h_affine+0xdf9>
  6804fe:	62 61 fe 48 6f 3f    	vmovdqu64 zmm31,ZMMWORD PTR [rdi]
  680504:	62 61 fe 48 6f 77 01 	vmovdqu64 zmm30,ZMMWORD PTR [rdi+0x40]
  68050b:	62 61 fe 48 6f 6f 04 	vmovdqu64 zmm29,ZMMWORD PTR [rdi+0x100]
  680512:	62 61 fe 48 6f 67 05 	vmovdqu64 zmm28,ZMMWORD PTR [rdi+0x140]
  680519:	62 61 fe 48 6f 5f 08 	vmovdqu64 zmm27,ZMMWORD PTR [rdi+0x200]
  680520:	62 61 fe 48 6f 57 09 	vmovdqu64 zmm26,ZMMWORD PTR [rdi+0x240]
  680527:	62 61 fe 48 6f 4f 0c 	vmovdqu64 zmm25,ZMMWORD PTR [rdi+0x300]
  68052e:	62 61 fe 48 6f 47 0d 	vmovdqu64 zmm24,ZMMWORD PTR [rdi+0x340]
  680535:	62 e1 fe 48 6f 7f 10 	vmovdqu64 zmm23,ZMMWORD PTR [rdi+0x400]
  68053c:	62 e1 fe 48 6f 77 11 	vmovdqu64 zmm22,ZMMWORD PTR [rdi+0x440]
  680543:	62 e1 fe 48 6f 6f 14 	vmovdqu64 zmm21,ZMMWORD PTR [rdi+0x500]
  68054a:	62 e1 fe 48 6f 67 15 	vmovdqu64 zmm20,ZMMWORD PTR [rdi+0x540]
  680551:	62 e1 fe 48 6f 5f 18 	vmovdqu64 zmm19,ZMMWORD PTR [rdi+0x600]
  680558:	62 e1 fe 48 6f 57 19 	vmovdqu64 zmm18,ZMMWORD PTR [rdi+0x640]
  68055f:	62 e1 fe 48 6f 4f 1c 	vmovdqu64 zmm17,ZMMWORD PTR [rdi+0x700]
  680566:	62 e1 fe 48 6f 47 1d 	vmovdqu64 zmm16,ZMMWORD PTR [rdi+0x740]
  68056d:	62 71 fe 48 6f 7f 20 	vmovdqu64 zmm15,ZMMWORD PTR [rdi+0x800]
  680574:	62 71 fe 48 6f 77 21 	vmovdqu64 zmm14,ZMMWORD PTR [rdi+0x840]
  68057b:	62 71 fe 48 6f 6f 24 	vmovdqu64 zmm13,ZMMWORD PTR [rdi+0x900]
  680582:	62 71 fe 48 6f 67 25 	vmovdqu64 zmm12,ZMMWORD PTR [rdi+0x940]
  680589:	62 71 fe 48 6f 5f 28 	vmovdqu64 zmm11,ZMMWORD PTR [rdi+0xa00]
  680590:	62 71 fe 48 6f 57 29 	vmovdqu64 zmm10,ZMMWORD PTR [rdi+0xa40]
  680597:	62 71 fe 48 6f 4f 2c 	vmovdqu64 zmm9,ZMMWORD PTR [rdi+0xb00]
  68059e:	62 71 fe 48 6f 47 2d 	vmovdqu64 zmm8,ZMMWORD PTR [rdi+0xb40]
  6805a5:	62 f1 fe 48 6f 7f 30 	vmovdqu64 zmm7,ZMMWORD PTR [rdi+0xc00]
  6805ac:	62 f1 fe 48 6f 77 31 	vmovdqu64 zmm6,ZMMWORD PTR [rdi+0xc40]
  6805b3:	62 f1 fe 48 6f 6f 34 	vmovdqu64 zmm5,ZMMWORD PTR [rdi+0xd00]
  6805ba:	62 f1 fe 48 6f 67 35 	vmovdqu64 zmm4,ZMMWORD PTR [rdi+0xd40]
  6805c1:	62 f1 7c 48 10 47 38 	vmovups zmm0,ZMMWORD PTR [rdi+0xe00]
  6805c8:	62 f1 7c 48 11 04 24 	vmovups ZMMWORD PTR [rsp],zmm0
  6805cf:	62 f1 fe 48 6f 47 39 	vmovdqu64 zmm0,ZMMWORD PTR [rdi+0xe40]
  6805d6:	62 f1 fe 48 7f 44 24 	vmovdqu64 ZMMWORD PTR [rsp+0x40],zmm0
  6805dd:	01 
  6805de:	62 f1 fe 48 6f 5f 3c 	vmovdqu64 zmm3,ZMMWORD PTR [rdi+0xf00]
  6805e5:	62 f1 fe 48 6f 57 3d 	vmovdqu64 zmm2,ZMMWORD PTR [rdi+0xf40]
  6805ec:	48 c1 e0 04          	shl    rax,0x4
  6805f0:	48 c1 e2 04          	shl    rdx,0x4
  6805f4:	48 83 e2 80          	and    rdx,0xffffffffffffff80
  6805f8:	48 8d 4c 0a 08       	lea    rcx,[rdx+rcx*1+0x8]
  6805fd:	48 8d 54 32 0f       	lea    rdx,[rdx+rsi*1+0xf]
  680602:	31 f6                	xor    esi,esi
  680604:	66 66 66 2e 0f 1f 84 	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  68060b:	00 00 00 00 00 
  680610:	62 f1 fe 48 7f 54 24 	vmovdqu64 ZMMWORD PTR [rsp+0x80],zmm2
  680617:	02 
  680618:	62 f2 7d 48 78 44 32 	vpbroadcastb zmm0,BYTE PTR [rdx+rsi*1-0xf]
  68061f:	f1 
  680620:	c4 e1 f8 90 54 31 f8 	kmovq  k2,QWORD PTR [rcx+rsi*1-0x8]
  680627:	c4 e1 f8 90 0c 31    	kmovq  k1,QWORD PTR [rcx+rsi*1]
  68062d:	62 f1 7f ca 6f c8    	vmovdqu8 zmm1{k2}{z},zmm0
  680633:	62 f1 05 40 ef d1    	vpxord zmm2,zmm31,zmm1
  680639:	62 f1 7f c9 6f c0    	vmovdqu8 zmm0{k1}{z},zmm0
  68063f:	62 61 0d 40 ef f8    	vpxord zmm31,zmm30,zmm0
  680645:	62 f2 7d 48 78 44 32 	vpbroadcastb zmm0,BYTE PTR [rdx+rsi*1-0xe]
  68064c:	f2 
  68064d:	62 f1 7f ca 6f c8    	vmovdqu8 zmm1{k2}{z},zmm0
  680653:	62 61 15 40 ef e9    	vpxord zmm29,zmm29,zmm1
  680659:	62 f1 7f c9 6f c0    	vmovdqu8 zmm0{k1}{z},zmm0
  68065f:	62 61 1d 40 ef e0    	vpxord zmm28,zmm28,zmm0
  680665:	62 f2 7d 48 78 44 32 	vpbroadcastb zmm0,BYTE PTR [rdx+rsi*1-0xd]
  68066c:	f3 
  68066d:	62 f1 7f ca 6f c8    	vmovdqu8 zmm1{k2}{z},zmm0
  680673:	62 61 25 40 ef d9    	vpxord zmm27,zmm27,zmm1
  680679:	62 f1 7f c9 6f c0    	vmovdqu8 zmm0{k1}{z},zmm0
  68067f:	62 61 2d 40 ef d0    	vpxord zmm26,zmm26,zmm0
  680685:	62 f2 7d 48 78 44 32 	vpbroadcastb zmm0,BYTE PTR [rdx+rsi*1-0xc]
  68068c:	f4 
  68068d:	62 f1 7f ca 6f c8    	vmovdqu8 zmm1{k2}{z},zmm0
  680693:	62 61 35 40 ef c9    	vpxord zmm25,zmm25,zmm1
  680699:	62 f1 7f c9 6f c0    	vmovdqu8 zmm0{k1}{z},zmm0
  68069f:	62 61 3d 40 ef c0    	vpxord zmm24,zmm24,zmm0
  6806a5:	62 f2 7d 48 78 44 32 	vpbroadcastb zmm0,BYTE PTR [rdx+rsi*1-0xb]
  6806ac:	f5 
  6806ad:	62 f1 7f ca 6f c8    	vmovdqu8 zmm1{k2}{z},zmm0
  6806b3:	62 e1 45 40 ef f9    	vpxord zmm23,zmm23,zmm1
  6806b9:	62 f1 7f c9 6f c0    	vmovdqu8 zmm0{k1}{z},zmm0
  6806bf:	62 e1 4d 40 ef f0    	vpxord zmm22,zmm22,zmm0
  6806c5:	62 f2 7d 48 78 44 32 	vpbroadcastb zmm0,BYTE PTR [rdx+rsi*1-0xa]
  6806cc:	f6 
  6806cd:	62 f1 7f ca 6f c8    	vmovdqu8 zmm1{k2}{z},zmm0
  6806d3:	62 e1 55 40 ef e9    	vpxord zmm21,zmm21,zmm1
  6806d9:	62 f1 7f c9 6f c0    	vmovdqu8 zmm0{k1}{z},zmm0
  6806df:	62 e1 5d 40 ef e0    	vpxord zmm20,zmm20,zmm0
  6806e5:	62 f2 7d 48 78 44 32 	vpbroadcastb zmm0,BYTE PTR [rdx+rsi*1-0x9]
  6806ec:	f7 
  6806ed:	62 f1 7f ca 6f c8    	vmovdqu8 zmm1{k2}{z},zmm0
  6806f3:	62 e1 65 40 ef d9    	vpxord zmm19,zmm19,zmm1
  6806f9:	62 f1 7f c9 6f c0    	vmovdqu8 zmm0{k1}{z},zmm0
  6806ff:	62 e1 6d 40 ef d0    	vpxord zmm18,zmm18,zmm0
  680705:	62 f2 7d 48 78 44 32 	vpbroadcastb zmm0,BYTE PTR [rdx+rsi*1-0x8]
  68070c:	f8 
  68070d:	62 f1 7f ca 6f c8    	vmovdqu8 zmm1{k2}{z},zmm0
  680713:	62 e1 75 40 ef c9    	vpxord zmm17,zmm17,zmm1
  680719:	62 f1 7f c9 6f c0    	vmovdqu8 zmm0{k1}{z},zmm0
  68071f:	62 e1 7d 40 ef c0    	vpxord zmm16,zmm16,zmm0
  680725:	62 f2 7d 48 78 44 32 	vpbroadcastb zmm0,BYTE PTR [rdx+rsi*1-0x7]
  68072c:	f9 
  68072d:	62 f1 7f ca 6f c8    	vmovdqu8 zmm1{k2}{z},zmm0
  680733:	62 71 05 48 ef f9    	vpxord zmm15,zmm15,zmm1
  680739:	62 f1 7f c9 6f c0    	vmovdqu8 zmm0{k1}{z},zmm0
  68073f:	62 71 0d 48 ef f0    	vpxord zmm14,zmm14,zmm0
  680745:	62 f2 7d 48 78 44 32 	vpbroadcastb zmm0,BYTE PTR [rdx+rsi*1-0x6]
  68074c:	fa 
  68074d:	62 f1 7f ca 6f c8    	vmovdqu8 zmm1{k2}{z},zmm0
  680753:	62 71 15 48 ef e9    	vpxord zmm13,zmm13,zmm1
  680759:	62 f1 7f c9 6f c0    	vmovdqu8 zmm0{k1}{z},zmm0
  68075f:	62 71 1d 48 ef e0    	vpxord zmm12,zmm12,zmm0
  680765:	62 f2 7d 48 78 44 32 	vpbroadcastb zmm0,BYTE PTR [rdx+rsi*1-0x5]
  68076c:	fb 
  68076d:	62 f1 7f ca 6f c8    	vmovdqu8 zmm1{k2}{z},zmm0
  680773:	62 71 25 48 ef d9    	vpxord zmm11,zmm11,zmm1
  680779:	62 f1 7f c9 6f c0    	vmovdqu8 zmm0{k1}{z},zmm0
  68077f:	62 71 2d 48 ef d0    	vpxord zmm10,zmm10,zmm0
  680785:	62 f2 7d 48 78 44 32 	vpbroadcastb zmm0,BYTE PTR [rdx+rsi*1-0x4]
  68078c:	fc 
  68078d:	62 f1 7f ca 6f c8    	vmovdqu8 zmm1{k2}{z},zmm0
  680793:	62 71 35 48 ef c9    	vpxord zmm9,zmm9,zmm1
  680799:	62 f1 7f c9 6f c0    	vmovdqu8 zmm0{k1}{z},zmm0
  68079f:	62 71 3d 48 ef c0    	vpxord zmm8,zmm8,zmm0
  6807a5:	62 f2 7d 48 78 44 32 	vpbroadcastb zmm0,BYTE PTR [rdx+rsi*1-0x3]
  6807ac:	fd 
  6807ad:	62 f1 7f ca 6f c8    	vmovdqu8 zmm1{k2}{z},zmm0
  6807b3:	62 f1 45 48 ef f9    	vpxord zmm7,zmm7,zmm1
  6807b9:	62 f1 7f c9 6f c0    	vmovdqu8 zmm0{k1}{z},zmm0
  6807bf:	62 f1 4d 48 ef f0    	vpxord zmm6,zmm6,zmm0
  6807c5:	62 f2 7d 48 78 44 32 	vpbroadcastb zmm0,BYTE PTR [rdx+rsi*1-0x2]
  6807cc:	fe 
  6807cd:	62 f1 7f ca 6f c8    	vmovdqu8 zmm1{k2}{z},zmm0
  6807d3:	62 f1 55 48 ef e9    	vpxord zmm5,zmm5,zmm1
  6807d9:	62 f1 7f c9 6f c0    	vmovdqu8 zmm0{k1}{z},zmm0
  6807df:	62 f1 5d 48 ef e0    	vpxord zmm4,zmm4,zmm0
  6807e5:	62 f2 7d 48 78 44 32 	vpbroadcastb zmm0,BYTE PTR [rdx+rsi*1-0x1]
  6807ec:	ff 
  6807ed:	62 f1 7f ca 6f c8    	vmovdqu8 zmm1{k2}{z},zmm0
  6807f3:	62 01 fd 48 6f f5    	vmovdqa64 zmm30,zmm29
  6807f9:	62 01 fd 48 6f ec    	vmovdqa64 zmm29,zmm28
  6807ff:	62 01 fd 48 6f e3    	vmovdqa64 zmm28,zmm27
  680805:	62 01 fd 48 6f da    	vmovdqa64 zmm27,zmm26
  68080b:	62 01 fd 48 6f d1    	vmovdqa64 zmm26,zmm25
  680811:	62 01 fd 48 6f c8    	vmovdqa64 zmm25,zmm24
  680817:	62 21 fd 48 6f c7    	vmovdqa64 zmm24,zmm23
  68081d:	62 a1 fd 48 6f fe    	vmovdqa64 zmm23,zmm22
  680823:	62 a1 fd 48 6f f5    	vmovdqa64 zmm22,zmm21
  680829:	62 a1 fd 48 6f ec    	vmovdqa64 zmm21,zmm20
  68082f:	62 a1 fd 48 6f e3    	vmovdqa64 zmm20,zmm19
  680835:	62 a1 fd 48 6f da    	vmovdqa64 zmm19,zmm18
  68083b:	62 a1 fd 48 6f d1    	vmovdqa64 zmm18,zmm17
  680841:	62 a1 fd 48 6f c8    	vmovdqa64 zmm17,zmm16
  680847:	62 c1 fd 48 6f c7    	vmovdqa64 zmm16,zmm15
  68084d:	62 51 fd 48 6f fe    	vmovdqa64 zmm15,zmm14
  680853:	62 51 fd 48 6f f5    	vmovdqa64 zmm14,zmm13
  680859:	62 51 fd 48 6f ec    	vmovdqa64 zmm13,zmm12
  68085f:	62 51 fd 48 6f e3    	vmovdqa64 zmm12,zmm11
  680865:	62 51 fd 48 6f da    	vmovdqa64 zmm11,zmm10
  68086b:	62 51 fd 48 6f d1    	vmovdqa64 zmm10,zmm9
  680871:	62 51 fd 48 6f c8    	vmovdqa64 zmm9,zmm8
  680877:	62 71 fd 48 6f c7    	vmovdqa64 zmm8,zmm7
  68087d:	62 f1 fd 48 6f fe    	vmovdqa64 zmm7,zmm6
  680883:	62 f1 fd 48 6f f5    	vmovdqa64 zmm6,zmm5
  680889:	62 f1 fd 48 6f ec    	vmovdqa64 zmm5,zmm4
  68088f:	62 f1 fe 48 6f 24 24 	vmovdqu64 zmm4,ZMMWORD PTR [rsp]
  680896:	62 f1 5d 48 ef e1    	vpxord zmm4,zmm4,zmm1
  68089c:	62 f1 fe 48 7f 24 24 	vmovdqu64 ZMMWORD PTR [rsp],zmm4
  6808a3:	62 f1 fd 48 6f e5    	vmovdqa64 zmm4,zmm5
  6808a9:	62 f1 fd 48 6f ee    	vmovdqa64 zmm5,zmm6
  6808af:	62 f1 fd 48 6f f7    	vmovdqa64 zmm6,zmm7
  6808b5:	62 d1 fd 48 6f f8    	vmovdqa64 zmm7,zmm8
  6808bb:	62 51 fd 48 6f c1    	vmovdqa64 zmm8,zmm9
  6808c1:	62 51 fd 48 6f ca    	vmovdqa64 zmm9,zmm10
  6808c7:	62 51 fd 48 6f d3    	vmovdqa64 zmm10,zmm11
  6808cd:	62 51 fd 48 6f dc    	vmovdqa64 zmm11,zmm12
  6808d3:	62 51 fd 48 6f e5    	vmovdqa64 zmm12,zmm13
  6808d9:	62 51 fd 48 6f ee    	vmovdqa64 zmm13,zmm14
  6808df:	62 51 fd 48 6f f7    	vmovdqa64 zmm14,zmm15
  6808e5:	62 31 fd 48 6f f8    	vmovdqa64 zmm15,zmm16
  6808eb:	62 a1 fd 48 6f c1    	vmovdqa64 zmm16,zmm17
  6808f1:	62 a1 fd 48 6f ca    	vmovdqa64 zmm17,zmm18
  6808f7:	62 a1 fd 48 6f d3    	vmovdqa64 zmm18,zmm19
  6808fd:	62 a1 fd 48 6f dc    	vmovdqa64 zmm19,zmm20
  680903:	62 a1 fd 48 6f e5    	vmovdqa64 zmm20,zmm21
  680909:	62 a1 fd 48 6f ee    	vmovdqa64 zmm21,zmm22
  68090f:	62 a1 fd 48 6f f7    	vmovdqa64 zmm22,zmm23
  680915:	62 81 fd 48 6f f8    	vmovdqa64 zmm23,zmm24
  68091b:	62 01 fd 48 6f c1    	vmovdqa64 zmm24,zmm25
  680921:	62 01 fd 48 6f ca    	vmovdqa64 zmm25,zmm26
  680927:	62 01 fd 48 6f d3    	vmovdqa64 zmm26,zmm27
  68092d:	62 01 fd 48 6f dc    	vmovdqa64 zmm27,zmm28
  680933:	62 01 fd 48 6f e5    	vmovdqa64 zmm28,zmm29
  680939:	62 01 fd 48 6f ee    	vmovdqa64 zmm29,zmm30
  68093f:	62 01 fd 48 6f f7    	vmovdqa64 zmm30,zmm31
  680945:	62 61 fd 48 6f fa    	vmovdqa64 zmm31,zmm2
  68094b:	62 f1 7f c9 6f c0    	vmovdqu8 zmm0{k1}{z},zmm0
  680951:	62 f1 fe 48 6f 4c 24 	vmovdqu64 zmm1,ZMMWORD PTR [rsp+0x40]
  680958:	01 
  680959:	62 f1 75 48 ef c8    	vpxord zmm1,zmm1,zmm0
  68095f:	62 f1 fe 48 7f 4c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x40],zmm1
  680966:	01 
  680967:	62 f2 7d 48 78 04 32 	vpbroadcastb zmm0,BYTE PTR [rdx+rsi*1]
  68096e:	62 f1 7f ca 6f c8    	vmovdqu8 zmm1{k2}{z},zmm0
  680974:	62 f1 65 48 ef d9    	vpxord zmm3,zmm3,zmm1
  68097a:	62 f1 7f c9 6f c0    	vmovdqu8 zmm0{k1}{z},zmm0
  680980:	62 f1 fe 48 6f 54 24 	vmovdqu64 zmm2,ZMMWORD PTR [rsp+0x80]
  680987:	02 
  680988:	62 f1 6d 48 ef d0    	vpxord zmm2,zmm2,zmm0
  68098e:	62 f1 fe 48 7f 54 24 	vmovdqu64 ZMMWORD PTR [rsp+0x80],zmm2
  680995:	02 
  680996:	62 f1 fe 48 6f 54 24 	vmovdqu64 zmm2,ZMMWORD PTR [rsp+0x80]
  68099d:	02 
  68099e:	48 83 c6 10          	add    rsi,0x10
  6809a2:	48 39 f0             	cmp    rax,rsi
  6809a5:	0f 85 65 fc ff ff    	jne    680610 <_RNvNtNtNtCs5Palesvl9wF_6mikros10univariate5tiled8affine_h16consume_h_affine+0x970>
  6809ab:	62 61 fe 48 7f 3f    	vmovdqu64 ZMMWORD PTR [rdi],zmm31
  6809b1:	62 61 fe 48 7f 77 01 	vmovdqu64 ZMMWORD PTR [rdi+0x40],zmm30
  6809b8:	62 61 fe 48 7f 6f 04 	vmovdqu64 ZMMWORD PTR [rdi+0x100],zmm29
  6809bf:	62 61 fe 48 7f 67 05 	vmovdqu64 ZMMWORD PTR [rdi+0x140],zmm28
  6809c6:	62 61 fe 48 7f 5f 08 	vmovdqu64 ZMMWORD PTR [rdi+0x200],zmm27
  6809cd:	62 61 fe 48 7f 57 09 	vmovdqu64 ZMMWORD PTR [rdi+0x240],zmm26
  6809d4:	62 61 fe 48 7f 4f 0c 	vmovdqu64 ZMMWORD PTR [rdi+0x300],zmm25
  6809db:	62 61 fe 48 7f 47 0d 	vmovdqu64 ZMMWORD PTR [rdi+0x340],zmm24
  6809e2:	62 e1 fe 48 7f 7f 10 	vmovdqu64 ZMMWORD PTR [rdi+0x400],zmm23
  6809e9:	62 e1 fe 48 7f 77 11 	vmovdqu64 ZMMWORD PTR [rdi+0x440],zmm22
  6809f0:	62 e1 fe 48 7f 6f 14 	vmovdqu64 ZMMWORD PTR [rdi+0x500],zmm21
  6809f7:	62 e1 fe 48 7f 67 15 	vmovdqu64 ZMMWORD PTR [rdi+0x540],zmm20
  6809fe:	62 e1 fe 48 7f 5f 18 	vmovdqu64 ZMMWORD PTR [rdi+0x600],zmm19
  680a05:	62 e1 fe 48 7f 57 19 	vmovdqu64 ZMMWORD PTR [rdi+0x640],zmm18
  680a0c:	62 e1 fe 48 7f 4f 1c 	vmovdqu64 ZMMWORD PTR [rdi+0x700],zmm17
  680a13:	62 e1 fe 48 7f 47 1d 	vmovdqu64 ZMMWORD PTR [rdi+0x740],zmm16
  680a1a:	62 71 fe 48 7f 7f 20 	vmovdqu64 ZMMWORD PTR [rdi+0x800],zmm15
  680a21:	62 71 fe 48 7f 77 21 	vmovdqu64 ZMMWORD PTR [rdi+0x840],zmm14
  680a28:	62 71 fe 48 7f 6f 24 	vmovdqu64 ZMMWORD PTR [rdi+0x900],zmm13
  680a2f:	62 71 fe 48 7f 67 25 	vmovdqu64 ZMMWORD PTR [rdi+0x940],zmm12
  680a36:	62 71 fe 48 7f 5f 28 	vmovdqu64 ZMMWORD PTR [rdi+0xa00],zmm11
  680a3d:	62 71 fe 48 7f 57 29 	vmovdqu64 ZMMWORD PTR [rdi+0xa40],zmm10
  680a44:	62 71 fe 48 7f 4f 2c 	vmovdqu64 ZMMWORD PTR [rdi+0xb00],zmm9
  680a4b:	62 71 fe 48 7f 47 2d 	vmovdqu64 ZMMWORD PTR [rdi+0xb40],zmm8
  680a52:	62 f1 fe 48 7f 7f 30 	vmovdqu64 ZMMWORD PTR [rdi+0xc00],zmm7
  680a59:	62 f1 fe 48 7f 77 31 	vmovdqu64 ZMMWORD PTR [rdi+0xc40],zmm6
  680a60:	62 f1 fe 48 7f 6f 34 	vmovdqu64 ZMMWORD PTR [rdi+0xd00],zmm5
  680a67:	62 f1 fe 48 7f 67 35 	vmovdqu64 ZMMWORD PTR [rdi+0xd40],zmm4
  680a6e:	62 f1 7c 48 10 04 24 	vmovups zmm0,ZMMWORD PTR [rsp]
  680a75:	62 f1 7c 48 11 47 38 	vmovups ZMMWORD PTR [rdi+0xe00],zmm0
  680a7c:	62 f1 7c 48 10 44 24 	vmovups zmm0,ZMMWORD PTR [rsp+0x40]
  680a83:	01 
  680a84:	62 f1 7c 48 11 47 39 	vmovups ZMMWORD PTR [rdi+0xe40],zmm0
  680a8b:	62 f1 fe 48 7f 5f 3c 	vmovdqu64 ZMMWORD PTR [rdi+0xf00],zmm3
  680a92:	62 f1 fe 48 7f 57 3d 	vmovdqu64 ZMMWORD PTR [rdi+0xf40],zmm2
  680a99:	48 81 c4 88 05 00 00 	add    rsp,0x588
  680aa0:	c5 f8 77             	vzeroupper 
  680aa3:	c3                   	ret    
  680aa4:	48 8d 0d 5d 3c 08 00 	lea    rcx,[rip+0x83c5d]        # 704708 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x888>
  680aab:	48 89 c7             	mov    rdi,rax
  680aae:	4c 89 c6             	mov    rsi,r8
  680ab1:	4c 89 c2             	mov    rdx,r8
  680ab4:	c5 f8 77             	vzeroupper 
  680ab7:	ff 15 13 6a 08 00    	call   QWORD PTR [rip+0x86a13]        # 7074d0 <_DYNAMIC+0x340>
  680abd:	cc                   	int3   
  680abe:	cc                   	int3   
  680abf:	cc                   	int3   

