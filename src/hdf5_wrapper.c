#include "hdf5.h"

int ex_create_hdf5_file(const char* fname) {

  hid_t       file_id;   /* file identifier */
  herr_t      status;

  /* Create a new file using default properties. */
  file_id = H5Fcreate(fname, H5F_ACC_TRUNC, H5P_DEFAULT, H5P_DEFAULT);

  /* Terminate access to the file. */
  status = H5Fclose(file_id); 
  return status;
}