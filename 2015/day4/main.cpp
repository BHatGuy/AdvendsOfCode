#include <openssl/md5.h>

#include <iostream>
#include <string>

using std::cout;
using std::endl;

const std::string INPUT = "ckczppom";

void print_hash(unsigned const char* hash) {
    for (size_t i = 0; i < MD5_DIGEST_LENGTH; i++) {
        std::printf("%02x", hash[i]);
    }
}

void solve(int places) {
    for (uint64_t i = 0;; i++) {
        std::string input = INPUT + std::to_string(i);
        unsigned char result[MD5_DIGEST_LENGTH];
        MD5(reinterpret_cast<unsigned const char*>(input.data()), input.size(), result);
        print_hash(result);
        bool ok = true;
        for (int j = 0; j < places; j++) {
            uint8_t byte = result[j / 2];
            uint8_t nibble;
            if (j % 2 == 0) {
                nibble = (byte & 0xf0) >> 4;
            } else {
                nibble = byte & 0x0f;
            }
            if (nibble != 0) {
                ok = false;
                break;
            }
        }
        if (ok) {
            cout << endl;
            cout << "Found " << input << endl;
            break;
        }
        cout << "\r";
    }
}

int main(int argc, char const* argv[]) {
    solve(5);
    solve(6);
    return 0;
}
