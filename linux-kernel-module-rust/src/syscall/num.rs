pub use crate::bindings::__NR_read as read;
pub use crate::bindings::__NR_write as write;
pub use crate::bindings::__NR_open as open;
pub use crate::bindings::__NR_close as close;
pub use crate::bindings::__NR_stat as stat;
pub use crate::bindings::__NR_fstat as fstat;
pub use crate::bindings::__NR_lstat as lstat;
pub use crate::bindings::__NR_poll as poll;
pub use crate::bindings::__NR_lseek as lseek;
pub use crate::bindings::__NR_mmap as mmap;
pub use crate::bindings::__NR_mprotect as mprotect;
pub use crate::bindings::__NR_munmap as munmap;
pub use crate::bindings::__NR_brk as brk;
pub use crate::bindings::__NR_rt_sigaction as rt_sigaction;
pub use crate::bindings::__NR_rt_sigprocmask as rt_sigprocmask;
pub use crate::bindings::__NR_rt_sigreturn as rt_sigreturn;
pub use crate::bindings::__NR_ioctl as ioctl;
pub use crate::bindings::__NR_pread64 as pread64;
pub use crate::bindings::__NR_pwrite64 as pwrite64;
pub use crate::bindings::__NR_readv as readv;
pub use crate::bindings::__NR_writev as writev;
pub use crate::bindings::__NR_access as access;
pub use crate::bindings::__NR_pipe as pipe;
pub use crate::bindings::__NR_select as select;
pub use crate::bindings::__NR_sched_yield as sched_yield;
pub use crate::bindings::__NR_mremap as mremap;
pub use crate::bindings::__NR_msync as msync;
pub use crate::bindings::__NR_mincore as mincore;
pub use crate::bindings::__NR_madvise as madvise;
pub use crate::bindings::__NR_shmget as shmget;
pub use crate::bindings::__NR_shmat as shmat;
pub use crate::bindings::__NR_shmctl as shmctl;
pub use crate::bindings::__NR_dup as dup;
pub use crate::bindings::__NR_dup2 as dup2;
pub use crate::bindings::__NR_pause as pause;
pub use crate::bindings::__NR_nanosleep as nanosleep;
pub use crate::bindings::__NR_getitimer as getitimer;
pub use crate::bindings::__NR_alarm as alarm;
pub use crate::bindings::__NR_setitimer as setitimer;
pub use crate::bindings::__NR_getpid as getpid;
pub use crate::bindings::__NR_sendfile as sendfile;
pub use crate::bindings::__NR_socket as socket;
pub use crate::bindings::__NR_connect as connect;
pub use crate::bindings::__NR_accept as accept;
pub use crate::bindings::__NR_sendto as sendto;
pub use crate::bindings::__NR_recvfrom as recvfrom;
pub use crate::bindings::__NR_sendmsg as sendmsg;
pub use crate::bindings::__NR_recvmsg as recvmsg;
pub use crate::bindings::__NR_shutdown as shutdown;
pub use crate::bindings::__NR_bind as bind;
pub use crate::bindings::__NR_listen as listen;
pub use crate::bindings::__NR_getsockname as getsockname;
pub use crate::bindings::__NR_getpeername as getpeername;
pub use crate::bindings::__NR_socketpair as socketpair;
pub use crate::bindings::__NR_setsockopt as setsockopt;
pub use crate::bindings::__NR_getsockopt as getsockopt;
pub use crate::bindings::__NR_clone as clone;
pub use crate::bindings::__NR_fork as fork;
pub use crate::bindings::__NR_vfork as vfork;
pub use crate::bindings::__NR_execve as execve;
pub use crate::bindings::__NR_exit as exit;
pub use crate::bindings::__NR_wait4 as wait4;
pub use crate::bindings::__NR_kill as kill;
pub use crate::bindings::__NR_uname as uname;
pub use crate::bindings::__NR_semget as semget;
pub use crate::bindings::__NR_semop as semop;
pub use crate::bindings::__NR_semctl as semctl;
pub use crate::bindings::__NR_shmdt as shmdt;
pub use crate::bindings::__NR_msgget as msgget;
pub use crate::bindings::__NR_msgsnd as msgsnd;
pub use crate::bindings::__NR_msgrcv as msgrcv;
pub use crate::bindings::__NR_msgctl as msgctl;
pub use crate::bindings::__NR_fcntl as fcntl;
pub use crate::bindings::__NR_flock as flock;
pub use crate::bindings::__NR_fsync as fsync;
pub use crate::bindings::__NR_fdatasync as fdatasync;
pub use crate::bindings::__NR_truncate as truncate;
pub use crate::bindings::__NR_ftruncate as ftruncate;
pub use crate::bindings::__NR_getdents as getdents;
pub use crate::bindings::__NR_getcwd as getcwd;
pub use crate::bindings::__NR_chdir as chdir;
pub use crate::bindings::__NR_fchdir as fchdir;
pub use crate::bindings::__NR_rename as rename;
pub use crate::bindings::__NR_mkdir as mkdir;
pub use crate::bindings::__NR_rmdir as rmdir;
pub use crate::bindings::__NR_creat as creat;
pub use crate::bindings::__NR_link as link;
pub use crate::bindings::__NR_unlink as unlink;
pub use crate::bindings::__NR_symlink as symlink;
pub use crate::bindings::__NR_readlink as readlink;
pub use crate::bindings::__NR_chmod as chmod;
pub use crate::bindings::__NR_fchmod as fchmod;
pub use crate::bindings::__NR_chown as chown;
pub use crate::bindings::__NR_fchown as fchown;
pub use crate::bindings::__NR_lchown as lchown;
pub use crate::bindings::__NR_umask as umask;
pub use crate::bindings::__NR_gettimeofday as gettimeofday;
pub use crate::bindings::__NR_getrlimit as getrlimit;
pub use crate::bindings::__NR_getrusage as getrusage;
pub use crate::bindings::__NR_sysinfo as sysinfo;
pub use crate::bindings::__NR_times as times;
pub use crate::bindings::__NR_ptrace as ptrace;
pub use crate::bindings::__NR_getuid as getuid;
pub use crate::bindings::__NR_syslog as syslog;
pub use crate::bindings::__NR_getgid as getgid;
pub use crate::bindings::__NR_setuid as setuid;
pub use crate::bindings::__NR_setgid as setgid;
pub use crate::bindings::__NR_geteuid as geteuid;
pub use crate::bindings::__NR_getegid as getegid;
pub use crate::bindings::__NR_setpgid as setpgid;
pub use crate::bindings::__NR_getppid as getppid;
pub use crate::bindings::__NR_getpgrp as getpgrp;
pub use crate::bindings::__NR_setsid as setsid;
pub use crate::bindings::__NR_setreuid as setreuid;
pub use crate::bindings::__NR_setregid as setregid;
pub use crate::bindings::__NR_getgroups as getgroups;
pub use crate::bindings::__NR_setgroups as setgroups;
pub use crate::bindings::__NR_setresuid as setresuid;
pub use crate::bindings::__NR_getresuid as getresuid;
pub use crate::bindings::__NR_setresgid as setresgid;
pub use crate::bindings::__NR_getresgid as getresgid;
pub use crate::bindings::__NR_getpgid as getpgid;
pub use crate::bindings::__NR_setfsuid as setfsuid;
pub use crate::bindings::__NR_setfsgid as setfsgid;
pub use crate::bindings::__NR_getsid as getsid;
pub use crate::bindings::__NR_capget as capget;
pub use crate::bindings::__NR_capset as capset;
pub use crate::bindings::__NR_rt_sigpending as rt_sigpending;
pub use crate::bindings::__NR_rt_sigtimedwait as rt_sigtimedwait;
pub use crate::bindings::__NR_rt_sigqueueinfo as rt_sigqueueinfo;
pub use crate::bindings::__NR_rt_sigsuspend as rt_sigsuspend;
pub use crate::bindings::__NR_sigaltstack as sigaltstack;
pub use crate::bindings::__NR_utime as utime;
pub use crate::bindings::__NR_mknod as mknod;
pub use crate::bindings::__NR_uselib as uselib;
pub use crate::bindings::__NR_personality as personality;
pub use crate::bindings::__NR_ustat as ustat;
pub use crate::bindings::__NR_statfs as statfs;
pub use crate::bindings::__NR_fstatfs as fstatfs;
pub use crate::bindings::__NR_sysfs as sysfs;
pub use crate::bindings::__NR_getpriority as getpriority;
pub use crate::bindings::__NR_setpriority as setpriority;
pub use crate::bindings::__NR_sched_setparam as sched_setparam;
pub use crate::bindings::__NR_sched_getparam as sched_getparam;
pub use crate::bindings::__NR_sched_setscheduler as sched_setscheduler;
pub use crate::bindings::__NR_sched_getscheduler as sched_getscheduler;
pub use crate::bindings::__NR_sched_get_priority_max as sched_get_priority_max;
pub use crate::bindings::__NR_sched_get_priority_min as sched_get_priority_min;
pub use crate::bindings::__NR_sched_rr_get_interval as sched_rr_get_interval;
pub use crate::bindings::__NR_mlock as mlock;
pub use crate::bindings::__NR_munlock as munlock;
pub use crate::bindings::__NR_mlockall as mlockall;
pub use crate::bindings::__NR_munlockall as munlockall;
pub use crate::bindings::__NR_vhangup as vhangup;
pub use crate::bindings::__NR_modify_ldt as modify_ldt;
pub use crate::bindings::__NR_pivot_root as pivot_root;
pub use crate::bindings::__NR__sysctl as _sysctl;
pub use crate::bindings::__NR_prctl as prctl;
pub use crate::bindings::__NR_arch_prctl as arch_prctl;
pub use crate::bindings::__NR_adjtimex as adjtimex;
pub use crate::bindings::__NR_setrlimit as setrlimit;
pub use crate::bindings::__NR_chroot as chroot;
pub use crate::bindings::__NR_sync as sync;
pub use crate::bindings::__NR_acct as acct;
pub use crate::bindings::__NR_settimeofday as settimeofday;
pub use crate::bindings::__NR_mount as mount;
pub use crate::bindings::__NR_umount2 as umount2;
pub use crate::bindings::__NR_swapon as swapon;
pub use crate::bindings::__NR_swapoff as swapoff;
pub use crate::bindings::__NR_reboot as reboot;
pub use crate::bindings::__NR_sethostname as sethostname;
pub use crate::bindings::__NR_setdomainname as setdomainname;
pub use crate::bindings::__NR_iopl as iopl;
pub use crate::bindings::__NR_ioperm as ioperm;
pub use crate::bindings::__NR_create_module as create_module;
pub use crate::bindings::__NR_init_module as init_module;
pub use crate::bindings::__NR_delete_module as delete_module;
pub use crate::bindings::__NR_get_kernel_syms as get_kernel_syms;
pub use crate::bindings::__NR_query_module as query_module;
pub use crate::bindings::__NR_quotactl as quotactl;
pub use crate::bindings::__NR_nfsservctl as nfsservctl;
pub use crate::bindings::__NR_getpmsg as getpmsg;
pub use crate::bindings::__NR_putpmsg as putpmsg;
pub use crate::bindings::__NR_afs_syscall as afs_syscall;
pub use crate::bindings::__NR_tuxcall as tuxcall;
pub use crate::bindings::__NR_security as security;
pub use crate::bindings::__NR_gettid as gettid;
pub use crate::bindings::__NR_readahead as readahead;
pub use crate::bindings::__NR_setxattr as setxattr;
pub use crate::bindings::__NR_lsetxattr as lsetxattr;
pub use crate::bindings::__NR_fsetxattr as fsetxattr;
pub use crate::bindings::__NR_getxattr as getxattr;
pub use crate::bindings::__NR_lgetxattr as lgetxattr;
pub use crate::bindings::__NR_fgetxattr as fgetxattr;
pub use crate::bindings::__NR_listxattr as listxattr;
pub use crate::bindings::__NR_llistxattr as llistxattr;
pub use crate::bindings::__NR_flistxattr as flistxattr;
pub use crate::bindings::__NR_removexattr as removexattr;
pub use crate::bindings::__NR_lremovexattr as lremovexattr;
pub use crate::bindings::__NR_fremovexattr as fremovexattr;
pub use crate::bindings::__NR_tkill as tkill;
pub use crate::bindings::__NR_time as time;
pub use crate::bindings::__NR_futex as futex;
pub use crate::bindings::__NR_sched_setaffinity as sched_setaffinity;
pub use crate::bindings::__NR_sched_getaffinity as sched_getaffinity;
pub use crate::bindings::__NR_set_thread_area as set_thread_area;
pub use crate::bindings::__NR_io_setup as io_setup;
pub use crate::bindings::__NR_io_destroy as io_destroy;
pub use crate::bindings::__NR_io_getevents as io_getevents;
pub use crate::bindings::__NR_io_submit as io_submit;
pub use crate::bindings::__NR_io_cancel as io_cancel;
pub use crate::bindings::__NR_get_thread_area as get_thread_area;
pub use crate::bindings::__NR_lookup_dcookie as lookup_dcookie;
pub use crate::bindings::__NR_epoll_create as epoll_create;
pub use crate::bindings::__NR_epoll_ctl_old as epoll_ctl_old;
pub use crate::bindings::__NR_epoll_wait_old as epoll_wait_old;
pub use crate::bindings::__NR_remap_file_pages as remap_file_pages;
pub use crate::bindings::__NR_getdents64 as getdents64;
pub use crate::bindings::__NR_set_tid_address as set_tid_address;
pub use crate::bindings::__NR_restart_syscall as restart_syscall;
pub use crate::bindings::__NR_semtimedop as semtimedop;
pub use crate::bindings::__NR_fadvise64 as fadvise64;
pub use crate::bindings::__NR_timer_create as timer_create;
pub use crate::bindings::__NR_timer_settime as timer_settime;
pub use crate::bindings::__NR_timer_gettime as timer_gettime;
pub use crate::bindings::__NR_timer_getoverrun as timer_getoverrun;
pub use crate::bindings::__NR_timer_delete as timer_delete;
pub use crate::bindings::__NR_clock_settime as clock_settime;
pub use crate::bindings::__NR_clock_gettime as clock_gettime;
pub use crate::bindings::__NR_clock_getres as clock_getres;
pub use crate::bindings::__NR_clock_nanosleep as clock_nanosleep;
pub use crate::bindings::__NR_exit_group as exit_group;
pub use crate::bindings::__NR_epoll_wait as epoll_wait;
pub use crate::bindings::__NR_epoll_ctl as epoll_ctl;
pub use crate::bindings::__NR_tgkill as tgkill;
pub use crate::bindings::__NR_utimes as utimes;
pub use crate::bindings::__NR_vserver as vserver;
pub use crate::bindings::__NR_mbind as mbind;
pub use crate::bindings::__NR_set_mempolicy as set_mempolicy;
pub use crate::bindings::__NR_get_mempolicy as get_mempolicy;
pub use crate::bindings::__NR_mq_open as mq_open;
pub use crate::bindings::__NR_mq_unlink as mq_unlink;
pub use crate::bindings::__NR_mq_timedsend as mq_timedsend;
pub use crate::bindings::__NR_mq_timedreceive as mq_timedreceive;
pub use crate::bindings::__NR_mq_notify as mq_notify;
pub use crate::bindings::__NR_mq_getsetattr as mq_getsetattr;
pub use crate::bindings::__NR_kexec_load as kexec_load;
pub use crate::bindings::__NR_waitid as waitid;
pub use crate::bindings::__NR_add_key as add_key;
pub use crate::bindings::__NR_request_key as request_key;
pub use crate::bindings::__NR_keyctl as keyctl;
pub use crate::bindings::__NR_ioprio_set as ioprio_set;
pub use crate::bindings::__NR_ioprio_get as ioprio_get;
pub use crate::bindings::__NR_inotify_init as inotify_init;
pub use crate::bindings::__NR_inotify_add_watch as inotify_add_watch;
pub use crate::bindings::__NR_inotify_rm_watch as inotify_rm_watch;
pub use crate::bindings::__NR_migrate_pages as migrate_pages;
pub use crate::bindings::__NR_openat as openat;
pub use crate::bindings::__NR_mkdirat as mkdirat;
pub use crate::bindings::__NR_mknodat as mknodat;
pub use crate::bindings::__NR_fchownat as fchownat;
pub use crate::bindings::__NR_futimesat as futimesat;
pub use crate::bindings::__NR_newfstatat as newfstatat;
pub use crate::bindings::__NR_unlinkat as unlinkat;
pub use crate::bindings::__NR_renameat as renameat;
pub use crate::bindings::__NR_linkat as linkat;
pub use crate::bindings::__NR_symlinkat as symlinkat;
pub use crate::bindings::__NR_readlinkat as readlinkat;
pub use crate::bindings::__NR_fchmodat as fchmodat;
pub use crate::bindings::__NR_faccessat as faccessat;
pub use crate::bindings::__NR_pselect6 as pselect6;
pub use crate::bindings::__NR_ppoll as ppoll;
pub use crate::bindings::__NR_unshare as unshare;
pub use crate::bindings::__NR_set_robust_list as set_robust_list;
pub use crate::bindings::__NR_get_robust_list as get_robust_list;
pub use crate::bindings::__NR_splice as splice;
pub use crate::bindings::__NR_tee as tee;
pub use crate::bindings::__NR_sync_file_range as sync_file_range;
pub use crate::bindings::__NR_vmsplice as vmsplice;
pub use crate::bindings::__NR_move_pages as move_pages;
pub use crate::bindings::__NR_utimensat as utimensat;
pub use crate::bindings::__NR_epoll_pwait as epoll_pwait;
pub use crate::bindings::__NR_signalfd as signalfd;
pub use crate::bindings::__NR_timerfd_create as timerfd_create;
pub use crate::bindings::__NR_eventfd as eventfd;
pub use crate::bindings::__NR_fallocate as fallocate;
pub use crate::bindings::__NR_timerfd_settime as timerfd_settime;
pub use crate::bindings::__NR_timerfd_gettime as timerfd_gettime;
pub use crate::bindings::__NR_accept4 as accept4;
pub use crate::bindings::__NR_signalfd4 as signalfd4;
pub use crate::bindings::__NR_eventfd2 as eventfd2;
pub use crate::bindings::__NR_epoll_create1 as epoll_create1;
pub use crate::bindings::__NR_dup3 as dup3;
pub use crate::bindings::__NR_pipe2 as pipe2;
pub use crate::bindings::__NR_inotify_init1 as inotify_init1;
pub use crate::bindings::__NR_preadv as preadv;
pub use crate::bindings::__NR_pwritev as pwritev;
pub use crate::bindings::__NR_rt_tgsigqueueinfo as rt_tgsigqueueinfo;
pub use crate::bindings::__NR_perf_event_open as perf_event_open;
pub use crate::bindings::__NR_recvmmsg as recvmmsg;
pub use crate::bindings::__NR_fanotify_init as fanotify_init;
pub use crate::bindings::__NR_fanotify_mark as fanotify_mark;
pub use crate::bindings::__NR_prlimit64 as prlimit64;
pub use crate::bindings::__NR_name_to_handle_at as name_to_handle_at;
pub use crate::bindings::__NR_open_by_handle_at as open_by_handle_at;
pub use crate::bindings::__NR_clock_adjtime as clock_adjtime;
pub use crate::bindings::__NR_syncfs as syncfs;
pub use crate::bindings::__NR_sendmmsg as sendmmsg;
pub use crate::bindings::__NR_setns as setns;
pub use crate::bindings::__NR_getcpu as getcpu;
pub use crate::bindings::__NR_process_vm_readv as process_vm_readv;
pub use crate::bindings::__NR_process_vm_writev as process_vm_writev;
pub use crate::bindings::__NR_kcmp as kcmp;
pub use crate::bindings::__NR_finit_module as finit_module;
pub use crate::bindings::__NR_sched_setattr as sched_setattr;
pub use crate::bindings::__NR_sched_getattr as sched_getattr;
pub use crate::bindings::__NR_renameat2 as renameat2;
pub use crate::bindings::__NR_seccomp as seccomp;
pub use crate::bindings::__NR_getrandom as getrandom;
pub use crate::bindings::__NR_memfd_create as memfd_create;
pub use crate::bindings::__NR_kexec_file_load as kexec_file_load;
pub use crate::bindings::__NR_bpf as bpf;
pub use crate::bindings::__NR_execveat as execveat;
pub use crate::bindings::__NR_userfaultfd as userfaultfd;
pub use crate::bindings::__NR_membarrier as membarrier;
pub use crate::bindings::__NR_mlock2 as mlock2;
pub use crate::bindings::__NR_copy_file_range as copy_file_range;
pub use crate::bindings::__NR_preadv2 as preadv2;
pub use crate::bindings::__NR_pwritev2 as pwritev2;
pub use crate::bindings::__NR_pkey_mprotect as pkey_mprotect;
pub use crate::bindings::__NR_pkey_alloc as pkey_alloc;
pub use crate::bindings::__NR_pkey_free as pkey_free;
pub use crate::bindings::__NR_statx as statx;