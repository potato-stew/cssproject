#define VSFTP_MAX_VISIT_REMEMBER 100
#define VSFTP_MAX_MSGFILE_SIZE 4000

#define VSFTP_STRING_HELPER

#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in_systm.h>
#include <netinet/in.h>
#include <netinet/ip.h>
#include <netinet/tcp.h>

typedef unsigned int (*hashfunc_t)(unsigned int, void*);

struct hash_node
{
  void* p_key;
  void* p_value;
  struct hash_node* p_prev;
  struct hash_node* p_next;
};

struct hash
{
  unsigned int buckets;
  unsigned int key_size;
  unsigned int value_size;
  hashfunc_t hash_func;
  struct hash_node** p_nodes;
};