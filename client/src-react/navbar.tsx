import React from 'react';
import Grid from '@mui/material/Grid';
import Box from '@mui/material/Box';

interface NavbarProps {
  Logo: any;
  FileDropdown: any;
  EditDropdown: any;
  ToolsDropdown: any;
}

function NavbarLayout({
  Logo,
  FileDropdown,
  EditDropdown,
  ToolsDropdown,
}: NavbarProps) {
  return (
      <Box sx={{ padding: "8px" }}>
      <Grid container>
        <Grid item xs={2}>
          { Logo }
        </Grid>

        <Grid item xs={2} />

        <Grid item xs={2}>
          { FileDropdown }
        </Grid>

        <Grid item xs={2}>
          { EditDropdown }
        </Grid>

        <Grid item xs={2}>
          { ToolsDropdown }
        </Grid>

      </Grid>
    </Box>
  );
}

export default (
  <NavbarLayout
    Logo="Logo"
    FileDropdown="File"
    EditDropdown="Edit"
    ToolsDropdown="Tools"
  />
);
