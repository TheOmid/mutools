import * as React from 'react';
import Paper from '@mui/material/Paper';
import MenuList from '@mui/material/MenuList';
import MenuItem from '@mui/material/MenuItem';
import ListItemText from '@mui/material/ListItemText';
import ListItemIcon from '@mui/material/ListItemIcon';
import Typography from '@mui/material/Typography';

export interface DropdownMenuItem {
  name: string;
  subtitle?: string;
  on_click: React.MouseEventHandler<HTMLButtonElement>;
}

export interface DropdownMenuProps {
  menu_items: [DropdownMenuItem];
}

function DropdownMenu({ menu_items }: DropdownMenuProps) {
  return (
    <Paper sx={{ width: 320, maxWidth: '100%' }}>
      <MenuList>

        {
				    menu_items.map((item) => (
                <MenuItem>
                    <ListItemIcon />
                    <ListItemText>{ item.name }</ListItemText>
                    <Typography variant="body2" color="text.secondary">
                    { item?.subtitle }
                </Typography>
                    </MenuItem>
				    ))
			  }
      
      </MenuList>
          </Paper>
  );
}

export default DropdownMenu;
