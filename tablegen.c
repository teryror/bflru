#include "stdio.h"

#define INDEX(a, b, c, d) (((a) << 6) | ((b) << 4) | ((c) << 2) | (d))

void main(int argc, char ** argv) {
    unsigned char ids[256];
    
    // Initialize to sentinel value for error checking
    for (int i = 0; i < 256; ++i) {
        ids[i] = 0xFF;
    }
    
    // Assign IDs to permutations of (0, 1, 2, 3)
    for (int a = 0; a < 4; ++a) {
        int id_part_b = 0;
        
        for (int b = 0; b < 4; ++b) {
            if (b == a) continue;
            int id_part_c = 0;
            
            for (int c = 0; c < 4; ++c) {
                if (c == a || c == b) continue;
                
                for (int d = 0; d < 4; ++d) {
                    if (d == a || d == b || d == c) continue;
                    
                    int index = INDEX(a, b, c, d);
                    if (index >= 256 || index < 0) {
                        printf("Assertion violated: Generated index out of range!\n"
                               "(%d, %d, %d, %d) -> %d\n", a, b, c, d, index);
                        return;
                    }
                    
                    int id = (12 * id_part_c) + (4 * id_part_b) + a;
                    if (id >= 24 || id < 0) {
                        printf("Assertion violated: Generated id out of range!\n"
                               "(%d, %d, %d, %d) -> %d\n", a, b, c, d, id);
                        return;
                    } else if ((id & 3) != a) {
                        printf("Assertion violated: Low bits of generated id do not match 'a'!\n"
                               "(%d, %d, %d, %d) -> %d\n", a, b, c, d, id);
                        return;
                    }
                    
                    ids[index] = (unsigned char) id;
                }
                
                id_part_c += 1;
            }
            
            id_part_b += 1;
        }
    }
    
    // Verify IDs are unique
    unsigned int bitmask = 0;
    for (int i = 0; i < 256; ++i) {
        if (ids[i] == 0xFF) continue;
        if (ids[i] < 24) {
            int bit = 1 << ids[i];
            
            if (bitmask & bit) {
                printf("Assertion violated: id %d is not unique!\n", ids[i]);
            } else {
                bitmask |= bit;
            }
        } else {
            // NOTE: This should _never_ happen, because we check for this when generating the ID...
            printf("Assertion violated: Generated id out of range!\n");
        }
    }
    
    // Initialize transition table with sentinel values
    unsigned char table[24 * 4];
    for (int i = 0; i < 24*4; ++i) {
        table[i] = 0xFF;
    }
    
    // Generate state transition table
    for (int a = 0; a < 4; ++a) {
        for (int b = 0; b < 4; ++b) {
            if (b == a) continue;
            
            for (int c = 0; c < 4; ++c) {
                if (c == a || c == b) continue;
                
                for (int d = 0; d < 4; ++d) {
                    if (d == a || d == b || d == c) continue;
                    
                    int id = ids[INDEX(a, b, c, d)];
                    table[id * 4 + 0] = ids[INDEX(a, b, c, d)];
                    table[id * 4 + 1] = ids[INDEX(b, a, c, d)];
                    table[id * 4 + 2] = ids[INDEX(c, a, b, d)];
                    table[id * 4 + 3] = ids[INDEX(d, a, b, c)];
                }
            }
        }
    }
    
    // Verify that the whole table is initialized
    for (int i = 0; i < 24*4; ++i) {
        if (table[i] == 0xFF) {
            printf("Assertion violated: Table was not filled completely!\n");
        }
    }
    
    // Print table
    printf("[\n");
    for (int state = 0; state < 24; ++state) {
        printf("    [");
        
        for (int transition = 0; transition < 4; ++transition) {
            printf("0x%02x", table[state * 4 + transition]);
            if (transition != 3) printf(", ");
        }
        
        if (state != 23) printf("],\n");
        else printf("]\n");
    }
    printf("]\n");
}