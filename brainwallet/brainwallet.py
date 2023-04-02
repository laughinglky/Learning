from getpass import getpass
from bitcoin import *

myBrainWalletPassword = getpass('password: ')
again = getpass('password again: ')

if not myBrainWalletPassword == again:
    print('mismatched passwords. quit...')
else:
    mySecretKey = encode_privkey(sha256(myBrainWalletPassword), "wif")
    myAddress = pubtoaddr(privtopub(mySecretKey))
    print('\n')
    print('secretKey:', '\033[37;47m' + mySecretKey + '\033[0m')
    print('  address:', myAddress)

    print('\n')
    print('verifying by signing... ',)
    test_msg = 'hello world'
    private_key = sha256(myBrainWalletPassword)
    sig = ecdsa_sign(test_msg, private_key)
    ownership = ecdsa_verify(test_msg, sig, myAddress)

    if ownership:
        print('passed.')
    else:
        print('NOT passed. !!! WARNING !!!')
